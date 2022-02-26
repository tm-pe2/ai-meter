use crate::schema::devices;

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
    duration: Option<Option<i32>>,
}
