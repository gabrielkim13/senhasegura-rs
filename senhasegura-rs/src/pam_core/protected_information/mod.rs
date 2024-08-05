mod access;
pub use access::*;

mod create;
pub use create::*;

mod disable;
pub use disable::*;

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
