// src/application/handlers/health.rs
use serde::Serialize;
use crate::application::response::ApiResponse;

#[derive(Serialize)]
pub struct HealthStatus {
    status: String,
    version: String,
}

pub async fn health_check() -> ApiResponse<HealthStatus> {
    let status = HealthStatus {
        status: "operational".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    ApiResponse::success(status)
}