use diesel::{dsl::sql, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    dto::MeterOutput,
    error::{Error, Result},
    model::{
        db::{CreateDbMeterData, DbMeterDbOutput, DbMeterDevice, UpdateDbMeterData},
        meter::Meter,
        Identifier,
    },
    schema::{devices, meterdevices, meters},
    PgPool,
};

impl Meter {
    pub(crate) async fn get_devices(id: i32, pool: &PgPool) -> Result<Vec<DbMeterDevice>> {
        let conn = pool.get()?;

        Ok(meterdevices::table
            .filter(meterdevices::meter.eq(id))
            .select((
                meterdevices::id,
                sql("devices.name"), // Hacky workaround
                sql("devices.consumption"),
                meterdevices::duration,
                meterdevices::turned_on,
            ))
            .inner_join(devices::table)
            .get_results(&conn)?)
    }
    pub(crate) async fn get_by_identifer(
        identifier: Identifier,
        pool: &PgPool,
    ) -> Result<MeterOutput> {
        match identifier {
            Identifier::Id(id) => Self::get_by_id(id, pool).await,
            Identifier::Name(_) => Err(Error::InvalidIdentifier),
        }
    }

    pub(crate) async fn get_by_id(id: i32, pool: &PgPool) -> Result<MeterOutput> {
        let conn = pool.get()?;

        let meter: DbMeterDbOutput = meters::table
            .select((
                meters::id,
                meters::occupants,
                meters::day_consumption,
                meters::night_consumption,
                meters::last_snapshot,
            ))
            .find(id)
            .first(&conn)?;

        Ok(MeterOutput::from((
            Some(Self::get_devices(meter.id, pool).await?),
            meter,
        )))
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<MeterOutput>> {
        let conn = pool.get()?;

        let a: Vec<DbMeterDbOutput> = meters::table
            .select((
                meters::id,
                meters::occupants,
                //sql("array_agg(meterdevices.id)"),
                meters::day_consumption,
                meters::night_consumption,
                meters::last_snapshot,
            ))
            //.left_join(meterdevices::table)
            //.group_by(meters::id)
            .load(&conn)?;

        let mut meters: Vec<MeterOutput> = Vec::with_capacity(a.len());

        for item in a {
            meters.push(MeterOutput::from((
                Some(Self::get_devices(item.id, pool).await?),
                item,
            )));
        }

        Ok(meters)
    }

    pub(crate) async fn create(data: CreateDbMeterData, pool: &PgPool) -> Result<MeterOutput> {
        let conn = pool.get()?;

        let meter: DbMeterDbOutput = diesel::insert_into(meters::table)
            .values(&data)
            .returning(meters::all_columns)
            .get_result(&conn)?;

        Ok(MeterOutput::from((None, meter)))
    }

    pub(crate) async fn update(
        identifier: Identifier,
        data: UpdateDbMeterData,
        pool: &PgPool,
    ) -> Result<MeterOutput> {
        let conn = pool.get()?;

        let id = match identifier {
            Identifier::Id(id) => id,
            Identifier::Name(_) => return Err(Error::InvalidIdentifier),
        };

        let meter: DbMeterDbOutput = diesel::update(meters::table.find(id))
            .set(&data)
            .get_result(&conn)?;

        Ok(MeterOutput::from((None, meter)))
    }
}
