use http::Method;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{Error, Response, SenhaseguraClient};

/// Access protected information API response.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct AccessProtectedInformationApiResponse {
    /// Response.
    pub response: Response,

    /// Access protected information result.
    pub info: AccessProtectedInformationResult,
}

/// Access protected information result (i.e. "info") field.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct AccessProtectedInformationResult {
    /// Protected information item ’s unique identification code.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,

    /// TODO: What is this?
    pub tag: Option<String>,

    /// Information type.
    pub r#type: Option<String>,

    /// Information you wish to protect.
    pub content: String,
}

/// Trait to access protected information.
///
/// See [Access protected information](https://docs.senhasegura.io/docs/a2a-pam-core-access-protected-information).
pub trait AccessProtectedInformationApi {
    /// Returns the protected information item.
    #[allow(async_fn_in_trait)]
    async fn access_protected_information(
        &self,
        id: i32,
    ) -> Result<AccessProtectedInformationApiResponse, Error>;
}

impl AccessProtectedInformationApi for SenhaseguraClient {
    #[tracing::instrument(level = "info", skip(self), err)]
    async fn access_protected_information(
        &self,
        id: i32,
    ) -> Result<AccessProtectedInformationApiResponse, Error> {
        self.do_api_request(Method::GET, &format!("iso/pam/info/{id}"), None::<()>)
            .await
    }
}
