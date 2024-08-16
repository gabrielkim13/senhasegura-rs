use std::ffi::c_char;

use crate::{
    c_char_to_string, handle_result, ApiError, Error, ErrorCode, Response, SenhaseguraClient,
    TryAssignFrom,
};

/// Disable protected information API response.
#[repr(C)]
pub struct DisableProtectedInformationApiResponse {
    /// Response.
    pub response: Response,
}

impl TryAssignFrom<senhasegura_rs::DisableProtectedInformationApiResponse>
    for DisableProtectedInformationApiResponse
{
    fn assign(
        &mut self,
        value: senhasegura_rs::DisableProtectedInformationApiResponse,
    ) -> Result<(), Error> {
        self.response.assign(value.response)?;

        Ok(())
    }
}

/// Disable protected information.
///
/// @see https://docs.senhasegura.io/docs/a2a-pam-core-disable-protected-information.
///
/// # Safety
///
/// - The `id` parameter must be a valid C-style string.
/// - The `response` parameter must be a valid pointer to an `DisableProtectedInformationApiResponse`
///   struct.
/// - The `error` parameter must be a valid pointer to an `ApiError` struct.
#[no_mangle]
pub unsafe extern "C" fn disable_protected_information(
    client: *const SenhaseguraClient,
    id: *const c_char,
    response: *mut DisableProtectedInformationApiResponse,
    error: *mut ApiError,
) -> ErrorCode {
    let client = match unsafe { client.as_ref() } {
        Some(client) => client,
        None => return ErrorCode::ClientNotInitialized,
    };

    let id = match c_char_to_string(id) {
        Ok(id) => id,
        Err(e) => return e.into(),
    };

    let response = match unsafe { response.as_mut() } {
        Some(response) => response,
        None => return ErrorCode::InvalidNullPointer,
    };

    let error = match unsafe { error.as_mut() } {
        Some(error) => error,
        None => return ErrorCode::InvalidNullPointer,
    };

    let result = client.0.disable_protected_information_sync(id);

    match handle_result(result, response, error) {
        Ok(_) => ErrorCode::Ok,
        Err(e) => e.into(),
    }
}
