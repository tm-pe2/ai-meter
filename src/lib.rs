#[macro_use]
extern crate serde;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub mod error;
pub mod handler;
pub mod model;
pub mod service;

pub fn app() -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .into_inner();

    Router::new()
        .nest("/meter", handler::meter::routes())
        .nest("/health", handler::health::routes())
        .layer(middleware_stack)
}
