#[derive(Serialize)]
pub struct Health {
    pub(crate) db: DbHealth,
}

#[derive(Serialize)]
pub(crate) enum DbHealth {
    Available,
    Unavailable,
}
