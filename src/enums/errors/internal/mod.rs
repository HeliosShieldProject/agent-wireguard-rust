pub mod io_error;
pub use io_error::IOError;

pub enum InternalError {
    IOError(IOError),
    InternalError,
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InternalError::IOError(error) => write!(f, "{}", error),
            InternalError::InternalError => write!(f, "Internal error"),
        }
    }
}

pub trait Error {
    fn as_internal(&self) -> InternalError;
}

pub fn to_internal<E: Error>(error: E) -> InternalError {
    error.as_internal()
}
