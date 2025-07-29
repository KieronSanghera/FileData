use axum::{http::StatusCode, response::IntoResponse};

pub async fn livez() -> impl IntoResponse {
    tracing::info!("Livez");
    StatusCode::OK
}

pub async fn readyz() -> impl IntoResponse {
    tracing::info!("Readyz");
    StatusCode::OK
}
