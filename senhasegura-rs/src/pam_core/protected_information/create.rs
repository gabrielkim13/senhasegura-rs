use async_trait::async_trait;
use http::Method;

use crate::{Error, Response, SenhaseguraClient};

/// Create protected information API request.
#[derive(serde::Serialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CreateProtectedInformationApiRequest {
    /// Name assigned to the protected item.
    pub name: Option<String>,

    /// Information you wish to protect.
    pub content: String,

    /// Unique string to identify the protected item.
    ///
    /// See [ProtectedInformationIdentifier](super::ProtectedInformationIdentifier).
    pub identifier: Option<String>,

    /// Information type.
    pub r#type: Option<String>,
}

/// Create protected information API response.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CreateProtectedInformationApiResponse {
    /// Response.
    pub response: Response,

    /// Create protected information result.
    pub info: CreateProtectedInformationResult,
}

/// Create protected information result (i.e. "info") field.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct CreateProtectedInformationResult {
    /// Name assigned to the protected item.
    pub name: Option<String>,

    /// Information type.
    pub r#type: Option<String>,

    /// Name of the service associated to the information.
    pub service: Option<String>,

    /// URL associated to the information.
    pub url: Option<String>,

    /// Information you wish to protect.
    pub content: String,

    /// Comma-separated ACL of users / groups.
    pub users_allowed: Option<String>,

    /// Unique string to identify the protected item.
    pub identifier: Option<String>,
}

/// Trait to create protected information.
///
/// See [Create protected information](https://docs.senhasegura.io/docs/a2a-pam-core-create-protected-information).
#[async_trait]
pub trait CreateProtectedInformationApi: Send + Sync {
    /// Creates a protected information item.
    async fn create_protected_information(
        &self,
        request: CreateProtectedInformationApiRequest,
    ) -> Result<CreateProtectedInformationApiResponse, Error>;
}

#[async_trait]
impl CreateProtectedInformationApi for SenhaseguraClient {
    #[tracing::instrument(level = "info", skip(self), err)]
    async fn create_protected_information(
        &self,
        request: CreateProtectedInformationApiRequest,
    ) -> Result<CreateProtectedInformationApiResponse, Error> {
        self.do_api_request(Method::POST, "iso/pam/info", Some(request))
            .await
    }
}

#[cfg(feature = "blocking")]
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl SenhaseguraClient {
    /// Creates a protected information item.
    pub fn create_protected_information_sync(
        &self,
        request: CreateProtectedInformationApiRequest,
    ) -> Result<CreateProtectedInformationApiResponse, Error> {
        self.async_runtime()?.block_on(
            <Self as CreateProtectedInformationApi>::create_protected_information(self, request),
        )
    }
}

#[cfg(feature = "napi")]
mod senhasegura_js {
    use napi_derive::napi;

    use super::*;

    #[napi]
    impl SenhaseguraClient {
        /// Creates a protected information item.
        #[napi(js_name = createProtectedInformation)]
        pub async fn js_create_protected_information(
            &self,
            request: CreateProtectedInformationApiRequest,
        ) -> napi::Result<CreateProtectedInformationApiResponse> {
            <Self as CreateProtectedInformationApi>::create_protected_information(self, request)
                .await
                .map_err(Into::into)
        }
    }
}
