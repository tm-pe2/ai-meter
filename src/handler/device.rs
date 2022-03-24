use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::{
    error::ApiResult,
    model::{
        db::{CreateDbDeviceData, DbDevice, UpdateDbDeviceData},
        IdentifierPath,
    },
    service::DeviceService,
    PgPool,
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:identifier", get(find_by).patch(update))
}

pub(crate) async fn create(
    Json(input): Json<CreateDbDeviceData>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<DbDevice>)> {
    let state = DeviceService::create(input, &pool).await?;

    Ok((StatusCode::CREATED, Json(state)))
}

pub(crate) async fn list(Extension(pool): Extension<PgPool>) -> ApiResult<Json<Vec<DbDevice>>> {
    Ok(Json(DeviceService::list(&pool).await?))
}

pub(crate) async fn find_by(
    Path(identifier): Path<IdentifierPath>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<DbDevice>> {
    Ok(Json(
        DeviceService::get_by_identifier(identifier.into(), &pool).await?,
    ))
}

pub(crate) async fn update(
    Path(identifier): Path<IdentifierPath>,
    Json(input): Json<UpdateDbDeviceData>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<DbDevice>> {
    Ok(Json(
        DeviceService::update(identifier.into(), input, &pool).await?,
    ))
}
