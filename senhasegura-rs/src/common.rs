use crate::PAMCoreExceptionCode;

/// HTTP status code.
///
/// Wrapper around [http::StatusCode] to implement custom traits.
#[derive(Debug)]
pub struct StatusCode(http::StatusCode);

impl std::ops::Deref for StatusCode {
    type Target = http::StatusCode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<http::StatusCode> for StatusCode {
    fn from(status: http::StatusCode) -> Self {
        Self(status)
    }
}

impl From<StatusCode> for http::StatusCode {
    fn from(status: StatusCode) -> Self {
        status.0
    }
}

impl PartialEq<http::StatusCode> for StatusCode {
    fn eq(&self, other: &http::StatusCode) -> bool {
        self.0 == *other
    }
}

impl<'de> serde::Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected};

        let v: u16 = serde::Deserialize::deserialize(deserializer)?;

        http::StatusCode::from_u16(v).map(StatusCode).map_err(|_| {
            Error::invalid_value(Unexpected::Unsigned(v as u64), &"an HTTP status code")
        })
    }
}

// TODO: Find a better way to conditionally derive NAPI-RS's traits for `Response` and `Exception`.
//
// As of now, cfg_attr doesn't work for attributes of conditional macros, thus we need to duplicate
// the code for the NAPI-RS feature (i.e. enabled / disabled).
//
// See: https://users.rust-lang.org/t/attribute-macro-confusion-in-pyo3/64832.

/// Response (i.e. "response") field.
#[derive(serde::Deserialize, Debug)]
#[cfg(feature = "napi")]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
#[napi_derive::napi(object)]
pub struct Response {
    /// HTTP status code.
    #[napi(ts_type = "number")]
    pub status: StatusCode,

    /// Response message.
    pub message: String,

    /// Flag to indicate whether an error occurred.
    pub error: bool,

    /// Error code.
    pub error_code: i32,
}

/// Response (i.e. "response") field.
#[derive(serde::Deserialize, Debug)]
#[cfg(not(feature = "napi"))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Response {
    /// HTTP status code.
    pub status: StatusCode,

    /// Response message.
    pub message: String,

    /// Flag to indicate whether an error occurred.
    pub error: bool,

    /// Error code.
    pub error_code: i32,
}

/// Senhasegura API exception codes.
#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum ExceptionCode {
    /// PAM Core exception code.
    PAMCore(PAMCoreExceptionCode),

    /// Unknown exception code.
    Unknown(u16),
}

impl From<ExceptionCode> for u16 {
    fn from(value: ExceptionCode) -> Self {
        use ExceptionCode::*;

        match value {
            PAMCore(PAMCoreExceptionCode::ProtectedInformation(code)) => code as u16,
            Unknown(code) => code,
        }
    }
}

impl From<u16> for ExceptionCode {
    fn from(value: u16) -> Self {
        use crate::ProtectedInformationExceptionCode;

        use self::{ExceptionCode::*, PAMCoreExceptionCode::*};

        if let Some(code) = ProtectedInformationExceptionCode::from_repr(value) {
            PAMCore(ProtectedInformation(code))
        } else {
            Unknown(value)
        }
    }
}

/// Exception (i.e. "exception") field.
#[derive(serde::Deserialize, Debug)]
#[cfg(feature = "napi")]
#[napi_derive::napi(object)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Exception {
    /// Exception code.
    #[napi(ts_type = "number")]
    pub code: ExceptionCode,

    /// Exception message.
    pub message: String,

    /// Exception detail.
    pub detail: Option<String>,
}

/// Exception (i.e. "exception") field.
#[derive(serde::Deserialize, Debug)]
#[cfg(not(feature = "napi"))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct Exception {
    /// Exception code.
    pub code: ExceptionCode,

    /// Exception message.
    pub message: String,

    /// Exception detail.
    pub detail: Option<String>,
}

#[cfg(feature = "napi")]
mod senhasegura_js {
    use napi::bindgen_prelude::*;

    use super::*;

    impl TypeName for StatusCode {
        fn type_name() -> &'static str {
            "StatusCode"
        }

        fn value_type() -> ValueType {
            ValueType::Number
        }
    }

    impl ToNapiValue for StatusCode {
        unsafe fn to_napi_value(env: sys::napi_env, value: Self) -> Result<sys::napi_value> {
            u16::to_napi_value(env, value.0.as_u16())
        }
    }

    impl FromNapiValue for StatusCode {
        unsafe fn from_napi_value(env: sys::napi_env, nvalue: sys::napi_value) -> Result<Self> {
            u16::from_napi_value(env, nvalue).and_then(|v| {
                http::StatusCode::from_u16(v)
                    .map(StatusCode)
                    .map_err(|e| Error::from_reason(e.to_string()))
            })
        }
    }

    impl ValidateNapiValue for StatusCode {}

    impl TypeName for ExceptionCode {
        fn type_name() -> &'static str {
            "ExceptionCode"
        }

        fn value_type() -> ValueType {
            ValueType::Number
        }
    }

    impl ToNapiValue for ExceptionCode {
        unsafe fn to_napi_value(env: sys::napi_env, value: Self) -> Result<sys::napi_value> {
            u16::to_napi_value(env, value.into())
        }
    }

    impl FromNapiValue for ExceptionCode {
        unsafe fn from_napi_value(env: sys::napi_env, nvalue: sys::napi_value) -> Result<Self> {
            u16::from_napi_value(env, nvalue).map(|v| v.into())
        }
    }

    impl ValidateNapiValue for ExceptionCode {}
}

#[cfg(feature = "uniffi")]
mod senhasegura_uniffi {
    use anyhow::anyhow;

    use crate::UniffiCustomTypeConverter;

    use super::*;

    uniffi::custom_type!(StatusCode, u16);

    impl UniffiCustomTypeConverter for StatusCode {
        type Builtin = u16;

        fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
            http::StatusCode::from_u16(val)
                .map(StatusCode)
                .map_err(|e| anyhow!(e))
        }

        fn from_custom(obj: Self) -> Self::Builtin {
            obj.0.as_u16()
        }
    }

    uniffi::custom_type!(ExceptionCode, u16);

    impl UniffiCustomTypeConverter for ExceptionCode {
        type Builtin = u16;

        fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
            Ok(val.into())
        }

        fn from_custom(obj: Self) -> Self::Builtin {
            obj.into()
        }
    }
}
