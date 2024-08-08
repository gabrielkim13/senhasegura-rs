use std::io;

use crate::{Exception, Response};

/// Errors that can occur when interacting with Senhasegura's API.
#[derive(thiserror::Error, Debug)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
#[cfg_attr(feature = "uniffi", uniffi(flat_error))]
pub enum Error {
    /// API error.
    ///
    /// This error occurs when a request to the API is successful, but returns an error status code
    /// (e.g. 4xx or 5xx).
    #[error(transparent)]
    Api(#[from] ApiError),

    /// Transport error.
    ///
    /// This error occurs when a request to the API is unsuccessful (e.g. network error).
    #[error(transparent)]
    Transport(reqwest::Error),

    /// Other error.
    ///
    /// This error occurs when an error is returned that does not fit into the other categories.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// API error response.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct ApiError {
    /// Response.
    pub response: Response,

    /// Exception.
    pub exception: Option<Exception>,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.response.message)
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            Self::Transport(err)
        } else {
            Self::Other(err.into())
        }
    }
}

#[cfg(feature = "retry")]
impl From<reqwest_middleware::Error> for Error {
    fn from(err: reqwest_middleware::Error) -> Self {
        use reqwest_middleware::Error::*;

        match err {
            Middleware(e) => Self::Other(e),
            Reqwest(e) => e.into(),
        }
    }
}

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
    use anyhow::anyhow;
    use napi::bindgen_prelude::*;

    use super::ApiError;

    impl TypeName for super::Error {
        fn type_name() -> &'static str {
            "Error"
        }

        fn value_type() -> ValueType {
            ValueType::Object
        }
    }

    impl ToNapiValue for super::Error {
        unsafe fn to_napi_value(env: sys::napi_env, value: Self) -> napi::Result<sys::napi_value> {
            use super::Error::*;

            let env_wrapper = Env::from(env);

            let mut obj = env_wrapper.create_object()?;

            match value {
                Api(e) => {
                    obj.set("$type", "ApiError")?;
                    obj.set("apiError", e)?;
                }
                Transport(e) => {
                    obj.set("$type", "Transport")?;
                    obj.set("transport", e.to_string())?;
                }
                Other(e) => {
                    obj.set("$type", "Other")?;
                    obj.set("other", e.to_string())?;
                }
            }

            napi::bindgen_prelude::Object::to_napi_value(env, obj)
        }
    }

    impl FromNapiValue for super::Error {
        unsafe fn from_napi_value(
            env: sys::napi_env,
            nvalue: sys::napi_value,
        ) -> napi::Result<Self> {
            let obj = Object::from_napi_value(env, nvalue)?;

            if let Some(error) = obj.get::<_, ApiError>("apiError")? {
                return Ok(super::Error::Api(error));
            }

            // Unfortunately, we can't restore the original error type, from reqwest.
            //
            // However, it's not very usual to convert a JS error back to a Rust error.
            if let Some(message) = obj.get::<_, String>("transport")? {
                let error = anyhow!(message);

                return Ok(super::Error::Other(error));
            }

            if let Some(message) = obj.get::<_, String>("other")? {
                let error = anyhow!(message);

                return Ok(super::Error::Other(error));
            }

            Err(napi::Error::from_reason(
                "Missing fields: apiError | transport | other",
            ))
        }
    }

    impl ValidateNapiValue for super::Error {}

    impl From<super::Error> for napi::Error {
        fn from(value: super::Error) -> Self {
            napi::Error::new(Status::GenericFailure, value)
        }
    }

    impl From<super::Error> for JsError {
        fn from(value: super::Error) -> Self {
            JsError::from(napi::Error::from(value))
        }
    }
}
