use crate::{model::health::DbHealth, PgPool};

impl DbHealth {
    pub(crate) async fn check(pool: &PgPool) -> Self {
        match pool.get() {
            Ok(_) => Self::Available,
            Err(_) => Self::Unavailable,
        }
    }
}
