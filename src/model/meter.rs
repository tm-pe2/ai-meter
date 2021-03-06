use std::{
    ops::{Add, Div},
    time::{Duration, SystemTime},
};

use crate::{error::MeterError, util::epoch};

/// Idle power consumption per occupant in kWh
const IDLE_CONSUMPTION: f32 = 10.0;

type MeterResult<T> = std::result::Result<T, MeterError>;

/// Meter struct
#[derive(Debug, Serialize)]
pub struct Meter {
    /// Last generated data point,
    /// this is used to generate the next one
    pub last_data_point: DataPoint,

    /// House where the meter is placed
    pub house: House,
}

impl Meter {
    /// Create a new [`Meter`] with a [`House`] of 4 occupants
    pub fn new() -> Self {
        Self {
            last_data_point: DataPoint::new(),
            house: House::new(4),
        }
    }

    /// Calulates the values at the current time and returns/snapshots them
    pub fn snapshot(&mut self) -> MeterResult<DataPoint> {
        let duration = self.last_data_point.datetime.elapsed()?;

        // TODO: split up in day & night
        // For this we need a way to get the start/end times of
        // the day and night 'shift'. After that it's just making
        // 2 snapshots per full day cycle (night and day)
        // and combining those

        let consumption = self.house.consume(duration)?;

        let last_data_point = self.last_data_point + (consumption, 0.0);

        self.last_data_point = last_data_point;

        Ok(last_data_point)
    }
}

impl Default for Meter {
    fn default() -> Self {
        Self::new()
    }
}

/// A snapshot of the meter at `datetime` with day/night consumption
#[derive(Clone, Copy, Debug, Serialize)]
pub struct DataPoint {
    /// Power consumption during the day in kWh
    pub day_consumption: f32,

    /// Power consumption during the night in kWh
    pub night_consumption: f32,

    /// Power consumption at this point,
    pub current_consumption: f32,

    /// Snapshot taken on
    #[serde(serialize_with = "epoch::system_time")]
    pub datetime: SystemTime,
}

impl DataPoint {
    /// Return a new [`DataPoint`] with nog data
    fn new() -> Self {
        Self {
            day_consumption: 0 as f32,
            night_consumption: 0 as f32,
            current_consumption: 0 as f32,
            datetime: SystemTime::now(),
        }
    }

    /// Create a new [`DataPoint`] with day/night and current comsumption
    /// and the current [`SystemTime`]
    fn with(day_consumption: f32, night_consumption: f32, current_consumption: f32) -> Self {
        Self {
            day_consumption,
            night_consumption,
            current_consumption,
            ..Self::default()
        }
    }
}

impl Default for DataPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl Add<(f32, f32)> for DataPoint {
    type Output = Self;

    fn add(self, (day_consumption, night_consumption): (f32, f32)) -> Self::Output {
        DataPoint::with(
            self.day_consumption + day_consumption,
            self.night_consumption + night_consumption,
            self.current_consumption,
        )
    }
}

/// Representations of a house with n occupants and [`Device`]s that
/// consume electricity
#[derive(Debug, Serialize)]
pub struct House {
    /// Number of occupants in the house
    pub occupants: i32,

    /// List of devices
    pub devices: Vec<Device>,
}

impl House {
    fn new(occupants: i32) -> Self {
        Self {
            occupants,
            devices: Vec::new(),
        }
    }
}

impl House {
    /// Cacluate the consumuption of this [`House`] over `duration`
    fn consume(&mut self, duration: Duration) -> MeterResult<f32> {
        // Get duration in hours
        let secs = duration.as_secs_f32();
        let hours: f32 = secs.div((60 * 60) as f32);

        // Calculate the comsupmtion over `duration`
        let device_consumption: f32 = self
            .devices
            .iter_mut()
            .map(|device| device.consume(duration).unwrap()) // TODO: Get rid of this unwrap
            .sum();

        // occupants of house + 1: A household with 2 occupants will not
        // have a double idle consumption
        let household_consumption = (self
            .occupants
            .checked_add(1)
            .ok_or(MeterError::Calculation)?) as f32
            * IDLE_CONSUMPTION
            * hours;

        Ok(device_consumption + household_consumption)
    }
}

// NOTE: maybe in the future a device can also produce electricty
/// Device that can be on for a duration or untill manually turned off
/// and consume electricity
#[derive(Debug, Serialize)]
pub struct Device {
    /// Name of the device
    pub name: String,

    /// How much the devices consumes in kWh
    pub consumption: f32,

    /// For how long the devices runs
    /// if `None` forever
    pub duration: Option<Duration>,

    /// Toggle to turn the device on
    pub on: bool,
}

impl Device {
    /// Cacluate the consumuption of this [`Device`] over `duration`
    fn consume(&mut self, duration: Duration) -> MeterResult<f32> {
        // Return 0 if the device is not onn
        if !self.on {
            return Ok(0.0);
        }

        // If there is a duration for the device
        // check if it smaller/equal to `duration`
        let duration = match self.duration {
            Some(d) if d <= duration => d,
            _ => duration,
        };

        let secs = duration.as_secs_f32();
        let hours: f32 = secs.div((60 * 60) as f32);

        // Update duration of device
        if let Some(ref mut d) = self.duration {
            *d -= duration;
        }

        Ok(self.consumption * hours)
    }
}
