use crate::{
    error::Result,
    model::health::{DbHealth, Health},
    PgPool,
};

pub struct HealthService;

impl HealthService {
    pub(crate) async fn get(pool: &PgPool) -> Result<Health> {
        Ok(Health {
            db: DbHealth::check(pool).await,
        })
    }
}
