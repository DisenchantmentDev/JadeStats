use std::error::Error;
use std::fmt::{self, Formatter};

#[derive(Debug, Clone)]
pub struct AppError {
    pub details: String,
}

#[allow(clippy::allow_attributes, clippy::todo)]
impl AppError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_owned(),
        }
    }
}

#[allow(clippy::allow_attributes, clippy::todo)]
impl fmt::Display for AppError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[allow(clippy::allow_attributes, clippy::todo)]
impl Error for AppError {
    fn description(&self) -> &str {
        todo!()
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self {
            details: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self {
            details: e.to_string(),
        }
    }
}

impl From<analyzer_core::api_error::ApiError> for AppError {
    fn from(e: analyzer_core::api_error::ApiError) -> Self {
        Self {
            details: e.to_string(),
        }
    }
}
