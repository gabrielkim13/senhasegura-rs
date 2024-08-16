use std::ffi::c_char;

use crate::{
    assign_str_to_c_char, c_char_to_string, handle_result, ApiError, Error, ErrorCode, Response,
    SenhaseguraClient, TryAssignFrom,
};

/// Create protected information API request.
#[repr(C)]
pub struct CreateProtectedInformationApiRequest {
    /// Name assigned to the protected item (optional).
    pub name: *const c_char,

    /// Information you wish to protect.
    pub content: *const c_char,

    /// Unique string to identify the protected item (optional).
    pub identifier: *const c_char,

    /// Information type (optional).
    pub r#type: *const c_char,
}

impl TryFrom<&CreateProtectedInformationApiRequest>
    for senhasegura_rs::CreateProtectedInformationApiRequest
{
    type Error = Error;

    fn try_from(value: &CreateProtectedInformationApiRequest) -> Result<Self, Self::Error> {
        let name = if value.name.is_null() {
            None
        } else {
            Some(c_char_to_string(value.name)?)
        };

        let content = c_char_to_string(value.content)?;

        let identifier = if value.identifier.is_null() {
            None
        } else {
            Some(c_char_to_string(value.identifier)?)
        };

        let r#type = if value.r#type.is_null() {
            None
        } else {
            Some(c_char_to_string(value.r#type)?)
        };

        Ok(Self {
            name,
            content,
            identifier,
            r#type,
        })
    }
}

/// Create protected information API response.
#[repr(C)]
pub struct CreateProtectedInformationApiResponse {
    /// Response.
    pub response: Response,

    /// Create protected information result.
    pub info: CreateProtectedInformationResult,
}

impl TryAssignFrom<senhasegura_rs::CreateProtectedInformationApiResponse>
    for CreateProtectedInformationApiResponse
{
    fn assign(
        &mut self,
        value: senhasegura_rs::CreateProtectedInformationApiResponse,
    ) -> Result<(), Error> {
        self.response.assign(value.response)?;
        self.info.assign(value.info)?;

        Ok(())
    }
}

/// Create protected information result (i.e. "info") field.
#[repr(C)]
pub struct CreateProtectedInformationResult {
    /// Name assigned to the protected item (optional).
    pub name: *mut c_char,

    /// Information type (optional).
    pub r#type: *mut c_char,

    /// Name of the service associated to the information (optional).
    pub service: *mut c_char,

    /// URL associated to the information (optional).
    pub url: *mut c_char,

    /// Information you wish to protect.
    pub content: *mut c_char,

    /// Comma-separated ACL of users / groups (optional).
    pub users_allowed: *mut c_char,

    /// Unique string to identify the protected item (optional).
    pub identifier: *mut c_char,
}

impl TryAssignFrom<senhasegura_rs::CreateProtectedInformationResult>
    for CreateProtectedInformationResult
{
    fn assign(
        &mut self,
        value: senhasegura_rs::CreateProtectedInformationResult,
    ) -> Result<(), Error> {
        if let Some(name) = value.name {
            assign_str_to_c_char(&name, self.name)?;
        }

        if let Some(r#type) = value.r#type {
            assign_str_to_c_char(&r#type, self.r#type)?;
        }

        if let Some(service) = value.service {
            assign_str_to_c_char(&service, self.service)?;
        }

        if let Some(url) = value.url {
            assign_str_to_c_char(&url, self.url)?;
        }

        assign_str_to_c_char(&value.content, self.content)?;

        if let Some(users_allowed) = value.users_allowed {
            assign_str_to_c_char(&users_allowed, self.users_allowed)?;
        }

        if let Some(identifier) = value.identifier {
            assign_str_to_c_char(&identifier, self.identifier)?;
        }

        Ok(())
    }
}

/// Create / update protected information.
///
/// @see https://docs.senhasegura.io/docs/a2a-pam-core-create-protected-information.
///
/// # Safety
///
/// - The `request` parameter must be a valid pointer to a`CreateProtectedInformationApiRequest`
///   struct.
/// - The `response` parameter must be a valid pointer to an `CreateProtectedInformationApiResponse`
///   struct.
/// - The `error` parameter must be a valid pointer to an `ApiError` struct.
#[no_mangle]
pub unsafe extern "C" fn create_protected_information(
    client: *const SenhaseguraClient,
    request: *const CreateProtectedInformationApiRequest,
    response: *mut CreateProtectedInformationApiResponse,
    error: *mut ApiError,
) -> ErrorCode {
    let client = match unsafe { client.as_ref() } {
        Some(client) => client,
        None => return ErrorCode::ClientNotInitialized,
    };

    let request = {
        let request = match unsafe { request.as_ref() } {
            Some(request) => request,
            None => return ErrorCode::InvalidNullPointer,
        };

        match senhasegura_rs::CreateProtectedInformationApiRequest::try_from(request) {
            Ok(request) => request,
            Err(e) => return e.into(),
        }
    };

    let response = match unsafe { response.as_mut() } {
        Some(response) => response,
        None => return ErrorCode::InvalidNullPointer,
    };

    let error = match unsafe { error.as_mut() } {
        Some(error) => error,
        None => return ErrorCode::InvalidNullPointer,
    };

    let result = client.0.create_protected_information_sync(request);

    match handle_result(result, response, error) {
        Ok(_) => ErrorCode::Ok,
        Err(e) => e.into(),
    }
}
