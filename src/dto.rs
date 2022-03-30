use crate::model::{db::DbMeterDevice, meter::DataPoint, Identifier};

#[derive(Debug, Serialize, Clone)]
pub struct MeterOutput {
    /// id
    pub id: i32,

    /// Last generated data point,
    /// this is used to generate the next one
    pub last_data_point: DataPoint,

    /// House where the meter is placed
    pub house: HouseOutput,
}

#[derive(Debug, Serialize, Clone)]
pub struct HouseOutput {
    pub occupants: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub devices: Vec<DbMeterDevice>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMeterInput {
    /// Number of occupants in the house
    pub occupants: i32,

    ///
    pub day_consumption: f32,

    ///
    pub night_consumption: f32,

    ///
    pub latitude: f32,

    ///
    pub longitude: f32,
}

#[derive(Debug, Deserialize)]
pub struct CreateMeterDeviceInput {
    ///
    pub device: Identifier,

    ///
    pub on: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMeterDeviceInput {
    ///
    pub on: Option<bool>,

    ///
    #[serde(
        default,
        skip_serializing_if = "option::is_none",
        with = "serde_with::rust::double_option"
    )]
    pub duration: Option<Option<i32>>,
}
