pub struct MeterService;

use crate::{
    dto::MeterOutput,
    error::Result,
    model::{
        db::{CreateDbMeterData, UpdateDbMeterData},
        meter::Meter,
        Identifier,
    },
    PgPool,
};

impl MeterService {
    pub(crate) async fn get_by_identifier(
        identifier: Identifier,
        pool: &PgPool,
    ) -> Result<MeterOutput> {
        Meter::get_by_identifer(identifier, pool).await
    }

    pub(crate) async fn create(input: CreateDbMeterData, pool: &PgPool) -> Result<MeterOutput> {
        Meter::create(input, pool).await
    }

    pub(crate) async fn update(
        identifier: Identifier,
        input: UpdateDbMeterData,
        pool: &PgPool,
    ) -> Result<MeterOutput> {
        Meter::update(identifier, input, pool).await
    }

    pub(crate) async fn list(pool: &PgPool) -> Result<Vec<MeterOutput>> {
        Meter::get_all(pool).await
    }
}
