use http::Method;

use crate::{Error, Response, SenhaseguraClient};

/// Create protected information API request.
#[derive(serde::Serialize, Debug)]
#[cfg_attr(feature = "retry", derive(Clone))]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct CreateProtectedInformationApiRequest {
    /// Name assigned to the protected item.
    pub name: Option<String>,

    /// Information you wish to protect.
    pub content: String,

    /// Unique string to identify the protected item.
    pub identifier: Option<String>,

    /// Information type.
    pub r#type: Option<String>,
}

/// Create protected information API response.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct CreateProtectedInformationApiResponse {
    /// Response.
    pub response: Response,

    /// Create protected information result.
    pub info: CreateProtectedInformationResult,
}

/// Create protected information result (i.e. "info") field.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct CreateProtectedInformationResult {
    /// Name assigned to the protected item.
    pub name: Option<String>,

    /// Information type.
    pub r#type: Option<String>,

    /// TODO: What is this?
    pub service: Option<String>,

    /// TODO: What is this?
    pub url: Option<String>,

    /// Information you wish to protect.
    pub content: String,

    /// TODO: What is this?
    pub users_allowed: Option<String>,

    /// Unique string to identify the protected item.
    pub identifier: Option<String>,
}

/// Trait to create protected information.
///
/// See [Create protected information](https://docs.senhasegura.io/docs/a2a-pam-core-create-protected-information).
pub trait CreateProtectedInformationApi {
    /// Creates a protected information item.
    fn create_protected_information(
        &self,
        request: CreateProtectedInformationApiRequest,
    ) -> Result<CreateProtectedInformationApiResponse, Error>;
}

impl CreateProtectedInformationApi for SenhaseguraClient {
    #[tracing::instrument(level = "info", skip(self), err)]
    fn create_protected_information(
        &self,
        request: CreateProtectedInformationApiRequest,
    ) -> Result<CreateProtectedInformationApiResponse, Error> {
        self.do_api_operation(Method::POST, "iso/pam/info", Some(request))
    }
}
