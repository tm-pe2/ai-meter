pub struct MeterService;

use crate::{error::Result, model::meter::Meter, PgPool};

impl MeterService {
    pub(crate) async fn list(pool: &PgPool) -> Result<Vec<Meter>> {
        todo!()
    }
}
