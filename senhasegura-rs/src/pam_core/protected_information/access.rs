use async_trait::async_trait;
use http::Method;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{Error, Response, SenhaseguraClient};

use super::ProtectedInformationIdentifier;

/// Access protected information API response.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AccessProtectedInformationApiResponse {
    /// Response.
    pub response: Response,

    /// Access protected information result.
    pub info: AccessProtectedInformationResult,
}

/// Access protected information result (i.e. "info") field.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct AccessProtectedInformationResult {
    /// Protected information item ’s unique identification code.
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: i32,

    /// Information identifier.
    pub tag: Option<String>,

    /// Information type.
    pub r#type: Option<String>,

    /// Information you wish to protect.
    pub content: String,
}

/// Trait to access protected information.
///
/// See [Access protected information](https://docs.senhasegura.io/docs/a2a-pam-core-access-protected-information).
#[async_trait]
pub trait AccessProtectedInformationApi: Send + Sync {
    /// Returns the protected information item.
    async fn access_protected_information(
        &self,
        id: impl Into<ProtectedInformationIdentifier> + std::fmt::Debug + Send,
    ) -> Result<AccessProtectedInformationApiResponse, Error>;
}

#[async_trait]
impl AccessProtectedInformationApi for SenhaseguraClient {
    #[tracing::instrument(level = "info", skip(self), err)]
    async fn access_protected_information(
        &self,
        id: impl Into<ProtectedInformationIdentifier> + std::fmt::Debug + Send,
    ) -> Result<AccessProtectedInformationApiResponse, Error> {
        self.do_api_request(
            Method::GET,
            format!("iso/pam/info/{}", id.into()),
            None::<()>,
        )
        .await
    }
}

#[cfg(feature = "napi")]
mod senhasegura_js {
    use napi_derive::napi;

    use super::*;

    #[napi]
    impl SenhaseguraClient {
        /// Returns the protected information item.
        #[napi(js_name = accessProtectedInformation)]
        pub async fn js_access_protected_information(
            &self,
            id: napi::Either<i32, String>,
        ) -> napi::Result<AccessProtectedInformationApiResponse> {
            <Self as AccessProtectedInformationApi>::access_protected_information(self, id)
                .await
                .map_err(Into::into)
        }
    }
}

#[cfg(feature = "uniffi")]
mod senhasegura_uniffi {
    use super::*;

    #[uniffi::export]
    impl SenhaseguraClient {
        /// Returns the protected information item.
        fn access_protected_information(
            &self,
            id: String,
        ) -> Result<AccessProtectedInformationApiResponse, Error> {
            self.async_runtime()?.block_on(
                <Self as AccessProtectedInformationApi>::access_protected_information(self, id),
            )
        }
    }
}
