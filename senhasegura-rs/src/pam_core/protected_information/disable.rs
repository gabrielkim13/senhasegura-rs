use http::Method;

use crate::{Error, Response, SenhaseguraClient};

/// Disable protected information API response.
#[derive(serde::Deserialize, Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
pub struct DisableProtectedInformationApiResponse {
    /// Response.
    pub response: Response,
}

/// Trait to disable protected information.
///
/// See [Disable protected information](https://docs.senhasegura.io/docs/a2a-pam-core-disable-protected-information).
pub trait DisableProtectedInformationApi {
    /// Disables the protected information item.
    fn disable_protected_information(
        &self,
        id: String,
    ) -> Result<DisableProtectedInformationApiResponse, Error>;
}

impl DisableProtectedInformationApi for SenhaseguraClient {
    #[tracing::instrument(level = "info", skip(self), err)]
    fn disable_protected_information(
        &self,
        id: String,
    ) -> Result<DisableProtectedInformationApiResponse, Error> {
        self.do_api_operation(Method::DELETE, &format!("iso/pam/info/{id}"), None::<()>)
    }
}
