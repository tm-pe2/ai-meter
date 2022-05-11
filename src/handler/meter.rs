use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};

use crate::{
    dto::MeterOutput,
    dto::{CreateMeterDeviceInput, CreateMeterInput, UpdateMeterDeviceInput},
    error::ApiResult,
    model::{
        db::{DbMeterDevice, UpdateDbMeterData},
        IdentifierPath,
    },
    service::{MeterDeviceService, MeterService},
    PgPool,
};

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:meter_identifier", get(find_by).patch(update))
        .route("/:meter_identifier/update", patch(update_readings))
        .route(
            "/:meter_identifier/device",
            get(list_meterdevices).post(create_meterdevice),
        )
        .route(
            "/:meter_identifier/device/:device_identifier",
            get(find_device_by).patch(update_meterdevice),
        )
}

pub(crate) async fn list(Extension(pool): Extension<PgPool>) -> ApiResult<Json<Vec<MeterOutput>>> {
    Ok(Json(MeterService::list(&pool).await?))
}

pub(crate) async fn create(
    Json(input): Json<CreateMeterInput>,
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

pub(crate) async fn find_device_by(
    Path((meter_identifier, device_identifier)): Path<(IdentifierPath, IdentifierPath)>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<DbMeterDevice>> {
    Ok(Json(
        MeterDeviceService::get_by_identifiers(
            meter_identifier.into(),
            device_identifier.into(),
            &pool,
        )
        .await?,
    ))
}

pub(crate) async fn create_meterdevice(
    Path(meter_identifier): Path<IdentifierPath>,
    Json(input): Json<CreateMeterDeviceInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<DbMeterDevice>)> {
    let meterdevice = MeterDeviceService::create(meter_identifier.into(), input, &pool).await?;

    Ok((StatusCode::CREATED, Json(meterdevice)))
}

pub(crate) async fn list_meterdevices(
    Path(_meter_identifier): Path<IdentifierPath>,
    Extension(_pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<Vec<DbMeterDevice>>)> {
    todo!();
}

pub(crate) async fn update_meterdevice(
    Path((meter_identifier, device_identifier)): Path<(IdentifierPath, IdentifierPath)>,
    Json(input): Json<UpdateMeterDeviceInput>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<(StatusCode, Json<DbMeterDevice>)> {
    let meterdevice = MeterDeviceService::update(
        meter_identifier.into(),
        device_identifier.into(),
        input,
        &pool,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(meterdevice)))
}

pub(crate) async fn update_readings(
    Path(identifier): Path<IdentifierPath>,
    Extension(pool): Extension<PgPool>,
) -> ApiResult<Json<MeterOutput>> {
    Ok(Json(
        MeterService::update_readings(identifier.into(), &pool).await?,
    ))
}
