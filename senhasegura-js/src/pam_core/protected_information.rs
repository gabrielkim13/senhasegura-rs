use senhasegura_rs::pam_core::protected_information::*;

use crate::SenhaseguraClient;

#[napi]
impl SenhaseguraClient {
    /// Returns the protected information item.
    #[napi]
    pub async fn access_protected_information(
        &self,
        id: i32,
    ) -> napi::Result<AccessProtectedInformationApiResponse> {
        Ok(self.client.access_protected_information(id).await?)
    }

    /// Creates a protected information item.
    #[napi]
    pub async fn create_protected_information(
        &self,
        request: CreateProtectedInformationApiRequest,
    ) -> napi::Result<CreateProtectedInformationApiResponse> {
        Ok(self.client.create_protected_information(request).await?)
    }

    /// Disables the protected information item.
    #[napi]
    pub async fn disable_protected_information(
        &self,
        id: String,
    ) -> napi::Result<DisableProtectedInformationApiResponse> {
        Ok(self.client.disable_protected_information(id).await?)
    }
}
