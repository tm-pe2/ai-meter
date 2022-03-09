#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

use axum::{AddExtensionLayer, Router};
use diesel::{pg::PgConnection, r2d2};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod dto;
pub mod error;
pub mod handler;
pub mod model;
pub mod schema;
pub mod service;
mod sql;
mod util;

pub type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn app(pg_pool: PgPool) -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(pg_pool))
        .into_inner();

    Router::new()
        .nest("/meter", handler::meter::routes())
        .nest("/device", handler::device::routes())
        .nest("/health", handler::health::routes())
        .layer(middleware_stack)
}
