use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    error::Result,
    model::{
        db::{CreateDbDeviceData, DbDevice, UpdateDbDeviceData},
        Identifier,
    },
    schema::devices,
    PgPool,
};

impl DbDevice {
    pub(crate) async fn get_by_identifer(identifier: Identifier, pool: &PgPool) -> Result<Self> {
        match identifier {
            Identifier::Id(id) => Self::get_by_id(id, pool).await,
            Identifier::Name(ref name) => Self::get_by_name(name, pool).await,
        }
    }

    async fn get_id_by_name(name: &str, pool: &PgPool) -> Result<i32> {
        let conn = pool.get()?;

        Ok(devices::table
            .filter(devices::name.eq(name))
            .select(devices::id)
            .first(&conn)?)
    }

    pub(crate) async fn get_by_id(id: i32, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(devices::table.find(id).first(&conn)?)
    }

    pub(crate) async fn get_by_name(name: &str, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(devices::table.filter(devices::name.eq(name)).first(&conn)?)
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        let conn = pool.get()?;

        Ok(devices::table.load(&conn)?)
    }

    pub(crate) async fn create(data: CreateDbDeviceData, pool: &PgPool) -> Result<Self> {
        let conn = pool.get()?;

        Ok(diesel::insert_into(devices::table)
            .values(&data)
            .returning(devices::all_columns)
            .get_result(&conn)?)
    }

    pub(crate) async fn update(
        identifier: Identifier,
        data: UpdateDbDeviceData,
        pool: &PgPool,
    ) -> Result<Self> {
        let conn = pool.get()?;

        let id = match identifier {
            Identifier::Id(id) => id,
            Identifier::Name(ref name) => Self::get_id_by_name(name, pool).await?,
        };

        Ok(diesel::update(devices::table.find(id))
            .set(&data)
            .get_result(&conn)?)
    }
}
