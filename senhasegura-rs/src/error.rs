use std::io;

use crate::{Exception, Response};

/// Errors that can occur when interacting with Senhasegura's API.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// API error.
    ///
    /// This error occurs when a request to the API is successful, but returns an error status code
    /// (e.g. 4xx or 5xx).
    #[error(transparent)]
    API(#[from] APIError),

    /// Transport error.
    ///
    /// This error occurs when a request to the API is unsuccessful (e.g. network error).
    #[error(transparent)]
    Transport(#[from] Box<ureq::Transport>),

    /// Other error.
    ///
    /// This error occurs when an error is returned that does not fit into the other categories.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// API error response.
#[derive(serde::Deserialize, Debug)]
pub struct APIError {
    /// Response.
    pub response: Response,

    /// Exception.
    pub exception: Option<Exception>,
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.response.message)
    }
}

impl std::error::Error for APIError {}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::Other(err.into())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Other(err.into())
    }
}

#[cfg(feature = "napi")]
mod senhasegura_js {
    use super::*;

    impl From<Error> for napi::Error {
        fn from(value: Error) -> Self {
            napi::Error::from_reason(value.to_string())
        }
    }
}
