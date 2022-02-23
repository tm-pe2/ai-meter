use axum::{routing::get, Json, Router};

use crate::{error::ApiResult, model::health::Health, service::HealthService};

pub(crate) fn routes() -> Router {
    Router::new().route("/", get(health))
}

pub(crate) async fn health() -> ApiResult<Json<Health>> {
    Ok(Json(HealthService::get()?))
}
