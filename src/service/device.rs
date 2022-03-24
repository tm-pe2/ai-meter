pub struct DeviceService;

use crate::{
    error::Result,
    model::{
        db::{CreateDbDeviceData, DbDevice, UpdateDbDeviceData},
        Identifier,
    },
    PgPool,
};

impl DeviceService {
    pub(crate) async fn create(input: CreateDbDeviceData, pool: &PgPool) -> Result<DbDevice> {
        DbDevice::create(input, pool).await
    }

    pub(crate) async fn update(
        identifier: Identifier,
        input: UpdateDbDeviceData,
        pool: &PgPool,
    ) -> Result<DbDevice> {
        DbDevice::update(identifier, input, pool).await
    }

    pub(crate) async fn get_by_identifier(
        identifier: Identifier,
        pool: &PgPool,
    ) -> Result<DbDevice> {
        DbDevice::get_by_identifer(identifier, pool).await
    }

    pub(crate) async fn list(pool: &PgPool) -> Result<Vec<DbDevice>> {
        DbDevice::get_all(pool).await
    }
}
