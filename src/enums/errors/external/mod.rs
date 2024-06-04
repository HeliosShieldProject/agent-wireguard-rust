pub mod io_error;
pub use io_error::IOError;

use super::internal;
use crate::dto::response::error::Response;
use axum::{http::StatusCode, response::IntoResponse};

pub enum ExternalError {
    InternalError,
    IOError(IOError),
}

pub trait Error {
    fn as_external(&self) -> ExternalError;
}

pub fn to_external<E: Error>(error: E) -> ExternalError {
    error.as_external()
}

impl Error for internal::InternalError {
    fn as_external(&self) -> ExternalError {
        match self {
            internal::InternalError::IOError(e) => e.as_external(),
            _ => ExternalError::InternalError,
        }
    }
}

impl IntoResponse for ExternalError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ExternalError::IOError(e) => e.into_response(),
            _ => Response {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
                error: "Internal",
            }
            .into_response(),
        }
    }
}
