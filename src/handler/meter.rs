use axum::{extract::Extension, Json, Router};

use crate::{error::ApiResult, model::meter::Meter, service::MeterService, PgPool};

pub(crate) fn routes() -> Router {
    Router::new()
}

pub(crate) async fn list(Extension(pool): Extension<PgPool>) -> ApiResult<Json<Vec<Meter>>> {
    Ok(Json(MeterService::list(&pool).await?))
}
