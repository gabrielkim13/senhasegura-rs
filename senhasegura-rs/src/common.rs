use http::StatusCode;

use crate::PAMCoreExceptionCode;

/// Response (i.e. "response") field.
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    /// HTTP status code.
    #[serde(deserialize_with = "deserialize_status_code")]
    pub status: StatusCode,

    /// Response message.
    pub message: String,

    /// Flag to indicate whether an error occurred.
    pub error: bool,
}

/// Senhasegura API exception codes.
#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum ExceptionCode {
    /// PAM Core exception code.
    PAMCore(PAMCoreExceptionCode),
}

/// Exception (i.e. "exception") field.
#[derive(serde::Deserialize, Debug)]
pub struct Exception {
    /// Exception code.
    pub code: ExceptionCode,

    /// Exception message.
    pub message: String,

    /// Exception detail.
    pub detail: Option<String>,
}

fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Unexpected};

    let v: u16 = serde::Deserialize::deserialize(deserializer)?;

    StatusCode::try_from(v)
        .map_err(|_| Error::invalid_value(Unexpected::Unsigned(v as u64), &"an HTTP status code"))
}
