use std::error::Error;
use std::fmt::{self, Formatter};

#[derive(Debug)]
pub struct ApiError {
    pub details: String,
}

impl ApiError {
    pub fn new(msg: &str) -> ApiError {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<std::io::Error> for ApiError {
    fn from(e: std::io::Error) -> Self {
        Self {
            details: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        Self {
            details: e.to_string(),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        Self {
            details: e.to_string(),
        }
    }
}
