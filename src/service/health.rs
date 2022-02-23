use crate::{error::Result, model::health::Health};

pub struct HealthService;

impl HealthService {
    pub fn get() -> Result<Health> {
        Ok(Health {
            place_holder: String::from("Good for now I guess"),
        })
    }
}
