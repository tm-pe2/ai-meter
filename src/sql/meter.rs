use crate::{error::Result, model::meter::Meter, PgPool};

impl Meter {
    pub(crate) async fn get_by_identifer(pool: &PgPool) -> Result<Self> {
        todo!()
    }

    pub(crate) async fn get_by_id(pool: &PgPool) -> Result<Self> {
        todo!()
    }

    pub(crate) async fn get_by_name(pool: &PgPool) -> Result<Self> {
        todo!()
    }

    pub(crate) async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        todo!()
    }

    pub(crate) async fn create(pool: &PgPool) -> Result<Self> {
        todo!()
    }

    pub(crate) async fn update(pool: &PgPool) -> Result<Self> {
        todo!()
    }
}
