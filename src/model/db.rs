use crate::{
    dto::{HouseOutput, MeterOutput},
    model::meter::{DataPoint, Device},
    schema::{devices, meters},
};

/// [`Device`] as stored in the db
#[derive(Debug, Queryable, Serialize)]
pub struct DbDevice {
    /// Id/PK of the device
    id: i32,

    /// Name of the device
    name: String,

    /// How much the devices consumes in kWh
    consumption: f32,

    /// For how long the devices runs
    /// if `None` forever
    duration: Option<i32>,
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
        skip_serializing_if = "Option::is_none",
        with = "serde_with::rust::double_option"
    )]
    duration: Option<Option<i32>>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "meters"]
pub struct CreateDbMeterData {
    ///
    occupants: i32,

    ///
    day_consumption: f32,

    ///
    night_consumption: f32,
}

/// [`Meter`] as stored in the db spread between `meters` and `meterdevices`
#[derive(Debug, Queryable, Serialize)]
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
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "meters"]
pub struct UpdateDbMeterData {
    ///
    occupants: Option<i32>,

    ///
    day_consumption: Option<f32>,

    ///
    night_consumption: Option<f32>,
}

#[derive(Debug, Queryable, Serialize)]
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
        let devices = match devices {
            Some(devices) => devices,
            None => Vec::with_capacity(0),
        };

        Self {
            id: meter.id,
            last_data_point: DataPoint {
                day_consumption: meter.day_consumption,
                night_consumption: meter.night_consumption,
                current_consumption: 0.0,
                datetime: std::time::SystemTime::now(),
            },
            house: HouseOutput {
                occupants: meter.occupants,
                devices,
            },
        }
    }
}
