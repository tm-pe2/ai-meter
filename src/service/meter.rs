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
            latitude: input.latitude,
            longitude: input.longitude,
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

    pub(crate) async fn update_readings(
        identifier: Identifier,
        pool: &PgPool,
    ) -> Result<MeterOutput> {
        let meter_output = Meter::get_by_identifer(identifier.clone(), pool).await?;

        let mut meter: Meter = meter_output.into();

        let datapoint = meter.snapshot()?;

        let data = UpdateDbMeterData {
            occupants: None,
            day_consumption: Some(datapoint.day_consumption),
            night_consumption: Some(datapoint.night_consumption),
            last_snapshot: Some(SystemTime::now()),
            latitude: None,
            longitude: None,
        };

        Meter::update(identifier, data, pool).await
    }

    pub(crate) async fn list(pool: &PgPool) -> Result<Vec<MeterOutput>> {
        Meter::get_all(pool).await
    }
}
