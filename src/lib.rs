use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, TraceLayer},
    ServiceBuilderExt,
};

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(_data): Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}

pub fn get_app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
}

/// Run server instance
pub async fn run(addr: SocketAddr, app: Router) {
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
