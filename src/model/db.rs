use std::time::SystemTime;

use crate::{
    dto::{HouseOutput, MeterOutput},
    model::meter::{DataPoint, Device, House, Meter},
    schema::{devices, meterdevices, meters},
};

/// [`Device`] as stored in the db
#[derive(Debug, Queryable, Serialize)]
pub struct DbDevice {
    /// Id/PK of the device
    pub id: i32,

    /// Name of the device
    pub name: String,

    /// How much the devices consumes in kWh
    pub consumption: f32,

    /// For how long the devices runs
    /// if `None` forever
    pub duration: Option<i32>,
}

/// Data structure required to create a `Device` in the db
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "devices"]
pub struct CreateDbDeviceData {
    /// Name of the device
    name: String,

    /// How much the devices consumes in kWh
    consumption: f32,

    /// For how long the devices runs
    /// if `None` forever
    duration: Option<i32>,
}

/// Data structure required to update a `Device` in the db
#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "devices"]
pub struct UpdateDbDeviceData {
    /// Name of the device
    name: Option<String>,

    /// How much the devices consumes in kWh
    consumption: Option<f32>,

    /// For how long the devices runs
    /// if `None` forever
    #[serde(
        default,
        skip_serializing_if = "option::is_none",
        with = "serde_with::rust::double_option"
    )]
    duration: Option<Option<i32>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "meters"]
pub struct CreateDbMeterData {
    ///
    pub occupants: i32,

    ///
    pub day_consumption: f32,

    ///
    pub night_consumption: f32,

    ///
    pub last_snapshot: SystemTime,

    ///
    pub latitude: f32,

    ///
    pub longitude: f32,
}

/// [`Meter`] as stored in the db spread between `meters` and `meterdevices`
#[derive(Debug, Queryable)]
pub struct DbMeterDbOutput {
    /// Id/PK of the
    pub id: i32,

    /// Number of occupants in the house
    pub occupants: i32,

    // /// Devices assigned to the house
    // pub meter_devices: Vec<i32>,
    ///
    pub day_consumption: f32,

    ///
    pub night_consumption: f32,

    ///
    pub last_snapshot: SystemTime,

    ///
    pub latitude: f32,

    ///
    pub longitude: f32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "meters"]
pub struct UpdateDbMeterData {
    ///
    pub occupants: Option<i32>,

    ///
    pub day_consumption: Option<f32>,

    ///
    pub night_consumption: Option<f32>,

    ///
    pub last_snapshot: Option<SystemTime>,

    ///
    pub latitude: Option<f32>,

    ///
    pub longitude: Option<f32>,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct DbMeterDevice {
    /// Id/PK of the
    pub id: i32,

    ///
    pub name: String,

    ///
    pub consumption: f32,

    ///
    pub duration: Option<i32>,

    pub on: bool,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "meterdevices"]
pub struct CreateMeterDeviceData {
    ///
    pub meter: i32,

    ///
    pub device: i32,

    ///
    pub turned_on: bool,

    ///
    pub duration: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "meterdevices"]
pub struct UpdateMeterDeviceData {
    ///
    pub meter: Option<i32>,

    ///
    pub device: Option<i32>,

    ///
    pub turned_on: Option<bool>,

    ///
    pub duration: Option<Option<i32>>,
}

impl From<DbMeterDevice> for Device {
    fn from(db_meter_device: DbMeterDevice) -> Self {
        let duration = db_meter_device.duration.map(|secs| {
            std::time::Duration::from_secs(secs.try_into().expect("Invalid duration in db"))
        });

        Self {
            name: db_meter_device.name,
            consumption: db_meter_device.consumption,
            duration,
            on: db_meter_device.on,
        }
    }
}

impl From<&DbMeterDevice> for Device {
    fn from(db_meter_device: &DbMeterDevice) -> Self {
        let duration = db_meter_device.duration.map(|secs| {
            std::time::Duration::from_secs(secs.try_into().expect("Invalid duration in db"))
        });

        Self {
            name: db_meter_device.name.clone(),
            consumption: db_meter_device.consumption,
            duration,
            on: db_meter_device.on,
        }
    }
}

impl From<(Option<Vec<DbMeterDevice>>, DbMeterDbOutput)> for MeterOutput {
    fn from((devices, meter): (Option<Vec<DbMeterDevice>>, DbMeterDbOutput)) -> Self {
        let (devices, current_consumption) = match devices {
            Some(devices) => {
                let mut current_consumption = 0.0;
                for device in devices.iter().filter(|&d| d.on) {
                    current_consumption += device.consumption;
                }
                (devices, current_consumption)
            }
            None => (Vec::with_capacity(0), 0.0),
        };

        Self {
            id: meter.id,
            last_data_point: DataPoint {
                day_consumption: meter.day_consumption,
                night_consumption: meter.night_consumption,
                current_consumption,
                datetime: meter.last_snapshot,
            },
            house: HouseOutput {
                occupants: meter.occupants,
                latitude: meter.latitude,
                longitude: meter.longitude,
                devices,
            },
        }
    }
}

impl From<MeterOutput> for Meter {
    fn from(meter_output: MeterOutput) -> Self {
        Self {
            last_data_point: DataPoint {
                day_consumption: meter_output.last_data_point.day_consumption,
                night_consumption: meter_output.last_data_point.night_consumption,
                current_consumption: meter_output.last_data_point.current_consumption,
                datetime: meter_output.last_data_point.datetime,
            },
            house: House {
                occupants: meter_output.house.occupants,
                devices: meter_output
                    .house
                    .devices
                    .iter()
                    .map(|d| d.into())
                    .collect(),
            },
        }
    }
}
