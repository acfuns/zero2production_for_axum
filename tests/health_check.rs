use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use zero2production_for_axum::get_app;

#[tokio::test]
async fn health_check_works() {
    let app = get_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_from_data() {
    let app = get_app();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/subscriptions")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert!(body.is_empty());
}
