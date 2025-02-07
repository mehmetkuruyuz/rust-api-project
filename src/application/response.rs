// src/application/response.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: Some(message.to_string()),
            data: Some(data),
        }
    }
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            errors: None,
        }
    }

    pub fn with_errors(message: &str, errors: Vec<String>) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            errors: Some(errors),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}