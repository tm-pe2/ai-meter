use std::net::SocketAddr;

use clap::Parser;
use diesel::{pg::PgConnection, r2d2};
use tokio::signal;

use meter::PgPool;

/// Server to simulate a meter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Socket address to listen on
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    addr: SocketAddr,

    #[clap(short, long, env)]
    database_url: String,

    /// Amount of info logged
    #[clap(short, long, possible_values = ["trace", "debug", "info", "warn", "error"])]
    log_level: Option<String>,
}

#[tokio::main]
async fn main() {
    // Load the `$PWD/.env` file on a debug build
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    // Get command line arguments
    let args = Args::parse();

    // Set log lvl if one is provided
    match args.log_level {
        Some(lvl) => std::env::set_var(
            "RUST_LOG",
            format!("meter={0},server={0},tower_http={0}", lvl),
        ),
        None => {
            // Set the RUST_LOG, if it hasn't been explicitly defined
            if std::env::var_os("RUST_LOG").is_none() {
                std::env::set_var("RUST_LOG", "meter=info,server=info,tower_http=info")
            }
        }
    };

    // Initiate logging to the terminal
    tracing_subscriber::fmt::init();

    let manager = r2d2::ConnectionManager::<PgConnection>::new(args.database_url);

    let pg_pool: PgPool = r2d2::Pool::builder()
        .build(manager)
        .expect("initial DB connection failed");

    let addr = args.addr;
    tracing::info!("listening on {addr}");
    let server = axum::Server::bind(&addr)
        .serve(meter::app(pg_pool).into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
