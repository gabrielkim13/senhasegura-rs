use senhasegura_rs::pam_core::protected_information::*;

use crate::SenhaseguraClient;

#[napi]
impl SenhaseguraClient {
    /// Returns the protected information item.
    #[napi]
    pub fn access_protected_information(
        &self,
        id: i32,
    ) -> napi::Result<AccessProtectedInformationApiResponse> {
        Ok(self.client.access_protected_information(id)?)
    }

    /// Creates a protected information item.
    #[napi]
    pub fn create_protected_information(
        &self,
        request: CreateProtectedInformationApiRequest,
    ) -> napi::Result<CreateProtectedInformationApiResponse> {
        Ok(self.client.create_protected_information(request)?)
    }

    /// Disables the protected information item.
    #[napi]
    pub fn disable_protected_information(
        &self,
        id: String,
    ) -> napi::Result<DisableProtectedInformationApiResponse> {
        Ok(self.client.disable_protected_information(id)?)
    }
}
