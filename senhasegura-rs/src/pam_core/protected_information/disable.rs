use http::Method;

use crate::{Error, Response, SenhaseguraClient};

/// Disable protected information API response.
#[derive(serde::Deserialize, Debug)]
pub struct DisableProtectedInformationAPIResponse {
    /// Response.
    pub response: Response,
}

/// Trait to disable protected information.
///
/// See [Disable protected information](https://docs.senhasegura.io/docs/a2a-pam-core-disable-protected-information).
pub trait DisableProtectedInformationAPI {
    /// Disables the protected information item.
    fn disable_protected_information(
        &self,
        id: String,
    ) -> Result<DisableProtectedInformationAPIResponse, Error>;
}

impl DisableProtectedInformationAPI for SenhaseguraClient {
    #[tracing::instrument(level = "info", skip(self), err)]
    fn disable_protected_information(
        &self,
        id: String,
    ) -> Result<DisableProtectedInformationAPIResponse, Error> {
        self.do_api_operation(Method::DELETE, &format!("iso/pam/info/{id}"), None::<()>)
    }
}
