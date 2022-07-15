use std::net::SocketAddr;

use tracing_subscriber::{fmt::time, layer::SubscriberExt, util::SubscriberInitExt};
use zero2production_for_axum::{get_app, run};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "zero2production_for_axum=debug,tower_http=debug".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_timer(time::LocalTime::rfc_3339()),
        )
        .init();

    tracing::info!("start web service.");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    let app = get_app();

    run(addr, app).await;
}
