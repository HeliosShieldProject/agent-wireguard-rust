use axum::{
    http::StatusCode,
    response::{self, IntoResponse},
};
use serde::Serialize;

use crate::dto::response::error::Response;

#[derive(Serialize)]
pub enum IOError {
    NotFoundError,
    PermissionDenied,
    ConnectionRefused,
}

impl IntoResponse for IOError {
    fn into_response(self) -> response::Response {
        let (status, message) = match self {
            IOError::NotFoundError => (StatusCode::INTERNAL_SERVER_ERROR, "File not found"),
            IOError::PermissionDenied => (StatusCode::INTERNAL_SERVER_ERROR, "Permission denied"),
            IOError::ConnectionRefused => (StatusCode::INTERNAL_SERVER_ERROR, "Connection refused"),
        };

        Response {
            status,
            message: message.to_string(),
            error: self,
        }
        .into_response()
    }
}
