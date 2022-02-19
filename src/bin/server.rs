use axum::{response::Json, routing::get, Router};
use clap::Parser;
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

/// Server to simulate a meter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Socket address to listen on
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    addr: SocketAddr,

    /// Amount of info logged
    #[clap(short, long, possible_values = ["trace", "debug", "info", "warn", "error"])]
    log_level: Option<String>,
}

#[tokio::main]
async fn main() {
    // Get command line arguments
    let args = Args::parse();

    // Set log lvl if one is provided
    match args.log_level {
        Some(lvl) => std::env::set_var("RUST_LOG", format!("server={0},tower_http={0}", lvl)),
        None => {
            // Set the RUST_LOG, if it hasn't been explicitly defined
            if std::env::var_os("RUST_LOG").is_none() {
                std::env::set_var("RUST_LOG", "server=info,tower_http=info")
            }
        }
    };

    // Initiate logging to the terminal
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http());

    // Run it
    let addr = args.addr;
    tracing::debug!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// This is just a template add more states when needed
/// Response to `/health`
#[derive(Serialize)]
struct Health {
    status: HealthStatus,
}

/// All possible health states
#[derive(Serialize)]
enum HealthStatus {
    /// Everything is working as it should
    Available,

    /// Something went wrong
    #[allow(dead_code)]
    Unavailable,
}

async fn health() -> Json<Health> {
    Json(Health {
        status: HealthStatus::Available,
    })
}
