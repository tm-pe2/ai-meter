use clap::Parser;
use std::net::SocketAddr;

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

    let addr = args.addr;
    tracing::debug!("listening on {addr}");
    let server = axum::Server::bind(&addr).serve(meter::app().into_make_service());

    if let Err(err) = server.await {
        tracing::error!("server error: {:?}", err);
    }
}
