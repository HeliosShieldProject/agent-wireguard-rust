use super::{Error, InternalError};
use crate::enums::errors::external;

pub enum IOError {
    NotFoundError,
    PermissionDenied,
    ConnectionRefused,
}

impl Error for std::io::Error {
    fn as_internal(&self) -> InternalError {
        match self.kind() {
            std::io::ErrorKind::NotFound => InternalError::IOError(IOError::NotFoundError),
            std::io::ErrorKind::PermissionDenied => {
                InternalError::IOError(IOError::PermissionDenied)
            }
            std::io::ErrorKind::ConnectionRefused => {
                InternalError::IOError(IOError::ConnectionRefused)
            }
            _ => InternalError::InternalError,
        }
    }
}

impl external::Error for IOError {
    fn as_external(&self) -> external::ExternalError {
        match self {
            IOError::NotFoundError => {
                external::ExternalError::IOError(external::IOError::NotFoundError)
            }
            IOError::PermissionDenied => {
                external::ExternalError::IOError(external::IOError::PermissionDenied)
            }
            IOError::ConnectionRefused => {
                external::ExternalError::IOError(external::IOError::ConnectionRefused)
            }
        }
    }
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IOError::NotFoundError => write!(f, "File not found"),
            IOError::PermissionDenied => write!(f, "Permission denied"),
            IOError::ConnectionRefused => write!(f, "Connection refused"),
        }
    }
}
