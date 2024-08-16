/// Senhasegura API client error codes.
#[derive(thiserror::Error, Debug)]
#[repr(C)]
pub enum ErrorCode {
    #[error("OK")]
    Ok,

    #[error("API error")]
    Api,

    #[error("Transport error")]
    Transport,

    #[error("Other error (Rust)")]
    Other,

    #[error("Client not initialized")]
    ClientNotInitialized,

    #[error("Invalid null pointer")]
    InvalidNullPointer,

    #[error("Invalid UTF-8 string")]
    InvalidString,

    #[error("Invalid URL")]
    InvalidUrl,
}

/// cbindgen:no-export
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(ErrorCode);

impl Error {
    pub fn api() -> Self {
        Self(ErrorCode::Api)
    }

    pub fn transport() -> Self {
        Self(ErrorCode::Transport)
    }

    pub fn other() -> Self {
        Self(ErrorCode::Other)
    }

    pub fn invalid_null_pointer() -> Self {
        Self(ErrorCode::InvalidNullPointer)
    }

    pub fn invalid_string() -> Self {
        Self(ErrorCode::InvalidString)
    }

    pub fn invalid_url() -> Self {
        Self(ErrorCode::InvalidUrl)
    }
}

impl From<Error> for ErrorCode {
    fn from(value: Error) -> Self {
        value.0
    }
}

impl<T> From<Result<T, Error>> for ErrorCode {
    fn from(value: Result<T, Error>) -> Self {
        match value {
            Ok(_) => ErrorCode::Ok,
            Err(e) => e.into(),
        }
    }
}

impl From<senhasegura_rs::Error> for Error {
    fn from(value: senhasegura_rs::Error) -> Self {
        match value {
            senhasegura_rs::Error::Api(_) => Error::api(),
            senhasegura_rs::Error::Transport(_) => Error::transport(),
            senhasegura_rs::Error::Other(_) => Error::other(),
        }
    }
}
