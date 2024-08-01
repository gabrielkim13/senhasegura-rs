/// Protected information API.
///
/// See [Protected Information API](https://docs.senhasegura.io/docs/a2a-pam-core-protected-information-api).
pub mod protected_information;
pub use protected_information::*;

/// PAM Core exception codes.
#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum PAMCoreExceptionCode {
    /// Protected information exception code.
    ProtectedInformation(ProtectedInformationExceptionCode),
}

/// Trait to interact with PAM Core APIs.
///
/// See [PAM Core APIs](https://docs.senhasegura.io/docs/a2a-apis-pam-core).
pub trait PAMCoreAPI: ProtectedInformationAPI {}

impl<T> PAMCoreAPI for T where T: ProtectedInformationAPI {}
