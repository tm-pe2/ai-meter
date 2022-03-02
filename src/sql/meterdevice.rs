use diesel::{dsl::sql, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    error::{Error, Result},
    model::{
        db::{CreateMeterDeviceData, DbMeterDevice, UpdateMeterDeviceData},
        Identifier,
    },
    schema::{devices, meterdevices},
    PgPool,
};

impl DbMeterDevice {
    pub(crate) async fn get_by_meter_id(id: i32, pool: &PgPool) -> Result<Vec<DbMeterDevice>> {
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

    pub(crate) async fn get_by_meter_device_id(
        meter_id: i32,
        meterdevice_id: i32,
        pool: &PgPool,
    ) -> Result<Self> {
        let conn = pool.get()?;

        Ok(meterdevices::table
            .find(meterdevice_id)
            .filter(meterdevices::meter.eq(meter_id))
            .select((
                meterdevices::id,
                sql("devices.name"), // Hacky workaround
                sql("devices.consumption"),
                meterdevices::duration,
                meterdevices::turned_on,
            ))
            .inner_join(devices::table)
            .first(&conn)?)
    }

    pub(crate) async fn get_by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(meterdevices::table
            .find(id)
            .select((
                meterdevices::id,
                sql("devices.name"), // Hacky workaround
                sql("devices.consumption"),
                meterdevices::duration,
                meterdevices::turned_on,
            ))
            .inner_join(devices::table)
            .first(&conn)?)
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        let conn = pool.get()?;

        Ok(meterdevices::table
            .select((
                meterdevices::id,
                sql("devices.name"), // Hacky workaround
                sql("devices.consumption"),
                meterdevices::duration,
                meterdevices::turned_on,
            ))
            .inner_join(devices::table)
            .load(&conn)?)
    }

    pub(crate) async fn create(data: CreateMeterDeviceData, pool: &PgPool) -> Result<i32> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(meterdevices::table)
            .values(&data)
            .returning(meterdevices::id)
            .get_result(&conn)?)
    }

    pub(crate) async fn update(
        identifier: Identifier,
        data: UpdateMeterDeviceData,
        pool: &PgPool,
    ) -> Result<i32> {
        let id = match identifier {
            Identifier::Id(id) => id,
            Identifier::Name(_) => return Err(Error::InvalidIdentifier),
        };

        let conn = pool.get()?;

        Ok(diesel::update(meterdevices::table.find(id))
            .set(&data)
            .returning(meterdevices::id)
            .get_result(&conn)?)
    }
}
