use std::time::SystemTime;

use crate::{
    dto::CreateMeterInput,
    dto::MeterOutput,
    error::Result,
    model::{
        db::{CreateDbMeterData, UpdateDbMeterData},
        meter::Meter,
        Identifier,
    },
    PgPool,
};

pub struct MeterService;

impl MeterService {
    pub(crate) async fn get_by_identifier(
        identifier: Identifier,
        pool: &PgPool,
    ) -> Result<MeterOutput> {
        Meter::get_by_identifer(identifier, pool).await
    }

    pub(crate) async fn create(input: CreateMeterInput, pool: &PgPool) -> Result<MeterOutput> {
        let data = CreateDbMeterData {
            occupants: input.occupants,
            day_consumption: input.day_consumption,
            night_consumption: input.night_consumption,
            last_snapshot: SystemTime::now(),
        };
        Meter::create(data, pool).await
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
