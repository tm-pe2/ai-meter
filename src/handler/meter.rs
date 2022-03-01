use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::{
    dto::MeterOutput,
    error::ApiResult,
    model::{
        db::{CreateDbMeterData, UpdateDbMeterData},
        IdentifierPath,
    },
    service::MeterService,
    PgPool,
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:identifier", get(find_by).patch(update))
}

pub(crate) async fn list(Extension(pool): Extension<PgPool>) -> ApiResult<Json<Vec<MeterOutput>>> {
    Ok(Json(MeterService::list(&pool).await?))
}

pub(crate) async fn create(
    Json(input): Json<CreateDbMeterData>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<MeterOutput>)> {
    let meter = MeterService::create(input, &pool).await?;

    Ok((StatusCode::CREATED, Json(meter)))
}

pub(crate) async fn find_by(
    Path(identifier): Path<IdentifierPath>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<MeterOutput>> {
    Ok(Json(
        MeterService::get_by_identifier(identifier.into(), &pool).await?,
    ))
}

pub(crate) async fn update(
    Path(identifier): Path<IdentifierPath>,
    Json(input): Json<UpdateDbMeterData>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<MeterOutput>> {
    Ok(Json(
        MeterService::update(identifier.into(), input, &pool).await?,
    ))
}
