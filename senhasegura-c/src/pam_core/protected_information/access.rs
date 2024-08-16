use std::ffi::c_char;

use crate::{
    assign_str_to_c_char, c_char_to_string, handle_result, ApiError, Error, ErrorCode, Response,
    SenhaseguraClient, TryAssignFrom,
};

/// Access protected information API response.
#[repr(C)]
pub struct AccessProtectedInformationApiResponse {
    /// Response.
    pub response: Response,

    /// Access protected information result.
    pub info: AccessProtectedInformationResult,
}

impl TryAssignFrom<senhasegura_rs::AccessProtectedInformationApiResponse>
    for AccessProtectedInformationApiResponse
{
    fn assign(
        &mut self,
        value: senhasegura_rs::AccessProtectedInformationApiResponse,
    ) -> Result<(), Error> {
        self.response.assign(value.response)?;
        self.info.assign(value.info)?;

        Ok(())
    }
}

/// Access protected information result (i.e. "info") field.
#[repr(C)]
pub struct AccessProtectedInformationResult {
    /// Protected information item ’s unique identification code.
    pub id: i32,

    /// Information identifier (optional).
    pub tag: *mut c_char,

    /// Information type (optional).
    pub r#type: *mut c_char,

    /// Information you wish to protect.
    pub content: *mut c_char,
}

impl TryAssignFrom<senhasegura_rs::AccessProtectedInformationResult>
    for AccessProtectedInformationResult
{
    fn assign(
        &mut self,
        value: senhasegura_rs::AccessProtectedInformationResult,
    ) -> Result<(), Error> {
        self.id = value.id;

        if let Some(tag) = value.tag {
            assign_str_to_c_char(&tag, self.tag)?;
        }

        if let Some(r#type) = value.r#type {
            assign_str_to_c_char(&r#type, self.r#type)?;
        }

        assign_str_to_c_char(&value.content, self.content)?;

        Ok(())
    }
}

/// Access protected information.
///
/// @see https://docs.senhasegura.io/docs/a2a-pam-core-access-protected-information.
///
/// # Safety
///
/// - The `id` parameter must be a valid C-style string.
/// - The `response` parameter must be a valid pointer to an `AccessProtectedInformationApiResponse`
///   struct.
/// - The `error` parameter must be a valid pointer to an `ApiError` struct.
#[no_mangle]
pub unsafe extern "C" fn access_protected_information(
    client: *const SenhaseguraClient,
    id: *const c_char,
    response: *mut AccessProtectedInformationApiResponse,
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

    let result = client.0.access_protected_information_sync(id);

    match handle_result(result, response, error) {
        Ok(_) => ErrorCode::Ok,
        Err(e) => e.into(),
    }
}
