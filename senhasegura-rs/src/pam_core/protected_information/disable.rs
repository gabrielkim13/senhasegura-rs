use async_trait::async_trait;
use http::Method;

use crate::{Error, Response, SenhaseguraClient};

use super::ProtectedInformationIdentifier;

/// Disable protected information API response.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct DisableProtectedInformationApiResponse {
    /// Response.
    pub response: Response,
}

/// Trait to disable protected information.
///
/// See [Disable protected information](https://docs.senhasegura.io/docs/a2a-pam-core-disable-protected-information).
#[async_trait]
pub trait DisableProtectedInformationApi: Send + Sync {
    /// Disables the protected information item.
    async fn disable_protected_information(
        &self,
        id: impl Into<ProtectedInformationIdentifier> + std::fmt::Debug + Send,
    ) -> Result<DisableProtectedInformationApiResponse, Error>;
}

#[async_trait]
impl DisableProtectedInformationApi for SenhaseguraClient {
    #[tracing::instrument(level = "info", skip(self), err)]
    async fn disable_protected_information(
        &self,
        id: impl Into<ProtectedInformationIdentifier> + std::fmt::Debug + Send,
    ) -> Result<DisableProtectedInformationApiResponse, Error> {
        self.do_api_request(
            Method::DELETE,
            format!("iso/pam/info/{}", id.into()),
            None::<()>,
        )
        .await
    }
}

#[cfg(feature = "blocking")]
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl SenhaseguraClient {
    /// Disables the protected information item.
    pub fn disable_protected_information_sync(
        &self,
        id: String,
    ) -> Result<DisableProtectedInformationApiResponse, Error> {
        self.async_runtime()?.block_on(
            <Self as DisableProtectedInformationApi>::disable_protected_information(self, id),
        )
    }
}

#[cfg(feature = "napi")]
mod senhasegura_js {
    use napi_derive::napi;

    use super::*;

    #[napi]
    impl SenhaseguraClient {
        /// Disables the protected information item.
        #[napi(js_name = disableProtectedInformation)]
        pub async fn js_disable_protected_information(
            &self,
            id: napi::Either<i32, String>,
        ) -> napi::Result<DisableProtectedInformationApiResponse> {
            <Self as DisableProtectedInformationApi>::disable_protected_information(self, id)
                .await
                .map_err(Into::into)
        }
    }
}
