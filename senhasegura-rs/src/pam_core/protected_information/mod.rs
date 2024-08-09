mod access;
pub use access::*;

mod create;
pub use create::*;

mod disable;
pub use disable::*;

/// Protected information identifier.
#[derive(Debug)]
pub enum ProtectedInformationIdentifier {
    /// Protected information item ’s unique identification code.
    Id(i32),

    /// Information identifier.
    Tag(String),
}

impl From<i32> for ProtectedInformationIdentifier {
    fn from(id: i32) -> Self {
        ProtectedInformationIdentifier::Id(id)
    }
}

impl PartialEq<i32> for ProtectedInformationIdentifier {
    fn eq(&self, other: &i32) -> bool {
        match self {
            ProtectedInformationIdentifier::Id(id) => id == other,
            _ => false,
        }
    }
}

impl From<String> for ProtectedInformationIdentifier {
    fn from(tag: String) -> Self {
        ProtectedInformationIdentifier::Tag(tag)
    }
}

impl PartialEq<String> for ProtectedInformationIdentifier {
    fn eq(&self, other: &String) -> bool {
        self.eq(&other.as_str())
    }
}

impl From<&str> for ProtectedInformationIdentifier {
    fn from(tag: &str) -> Self {
        tag.to_string().into()
    }
}

impl PartialEq<&str> for ProtectedInformationIdentifier {
    fn eq(&self, other: &&str) -> bool {
        match self {
            ProtectedInformationIdentifier::Tag(tag) => tag == *other,
            _ => false,
        }
    }
}

impl std::fmt::Display for ProtectedInformationIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtectedInformationIdentifier::Id(id) => write!(f, "{id}"),
            ProtectedInformationIdentifier::Tag(tag) => write!(f, "{tag}"),
        }
    }
}

/// Protected information exception codes.
#[derive(serde_repr::Deserialize_repr, strum::FromRepr, Debug)]
#[repr(u16)]
pub enum ProtectedInformationExceptionCode {
    /// Information not found.
    InformationNotFound = 1023,

    /// Inactive information.
    InactiveInformation = 1024,

    /// The information content was not informed.
    MissingContentParameter = 1026,
}

/// Trait to manage protected information.
///
/// See [Protected Information API](https://docs.senhasegura.io/docs/a2a-pam-core-protected-information-api).
pub trait ProtectedInformationApi:
    AccessProtectedInformationApi + CreateProtectedInformationApi + DisableProtectedInformationApi
{
}

impl<T> ProtectedInformationApi for T where
    T: AccessProtectedInformationApi
        + CreateProtectedInformationApi
        + DisableProtectedInformationApi
{
}

#[cfg(feature = "napi")]
mod senhasegura_js {
    use napi::bindgen_prelude::*;

    use super::*;

    impl TypeName for ProtectedInformationIdentifier {
        fn type_name() -> &'static str {
            "ProtectedInformationId"
        }

        fn value_type() -> ValueType {
            ValueType::Unknown
        }
    }

    impl ToNapiValue for ProtectedInformationIdentifier {
        unsafe fn to_napi_value(env: sys::napi_env, value: Self) -> Result<sys::napi_value> {
            use ProtectedInformationIdentifier::*;

            match value {
                Id(v) => i32::to_napi_value(env, v),
                Tag(v) => String::to_napi_value(env, v),
            }
        }
    }

    impl FromNapiValue for ProtectedInformationIdentifier {
        unsafe fn from_napi_value(env: sys::napi_env, nvalue: sys::napi_value) -> Result<Self> {
            if let Ok(v) = i32::from_napi_value(env, nvalue) {
                Ok(ProtectedInformationIdentifier::Id(v))
            } else {
                String::from_napi_value(env, nvalue).map(ProtectedInformationIdentifier::Tag)
            }
        }
    }

    impl ValidateNapiValue for ProtectedInformationIdentifier {}

    impl From<napi::Either<i32, String>> for ProtectedInformationIdentifier {
        fn from(id: napi::Either<i32, String>) -> Self {
            use napi::Either::*;

            match id {
                A(id) => ProtectedInformationIdentifier::Id(id),
                B(tag) => ProtectedInformationIdentifier::Tag(tag),
            }
        }
    }
}
