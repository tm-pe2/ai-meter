use crate::model::{db::DbMeterDevice, meter::DataPoint};

#[derive(Debug, Serialize)]
pub struct MeterOutput {
    /// id
    pub id: i32,

    /// Last generated data point,
    /// this is used to generate the next one
    pub last_data_point: DataPoint,

    /// House where the meter is placed
    pub house: HouseOutput,
}

#[derive(Debug, Serialize)]
pub struct HouseOutput {
    pub occupants: i32,
    pub devices: Vec<DbMeterDevice>,
}
