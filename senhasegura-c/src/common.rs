use std::ffi::c_char;

use crate::Error;

pub(crate) trait TryAssignFrom<T>
where
    Self: Sized,
{
    fn assign(&mut self, value: T) -> Result<(), Error>;
}

pub(crate) fn handle_result<TR, TC>(
    result: Result<TR, senhasegura_rs::Error>,
    response: &mut TC,
    error: &mut ApiError,
) -> Result<(), Error>
where
    TC: TryAssignFrom<TR>,
{
    match result {
        Ok(value) => {
            response.assign(value)?;

            Ok(())
        }
        Err(senhasegura_rs::Error::Api(api_error)) => {
            error.assign(api_error)?;

            Err(Error::api())
        }
        Err(e) => Err(e.into()),
    }
}

/// API error response.
#[repr(C)]
pub struct ApiError {
    /// Response.
    pub response: Response,

    /// Exception (optional).
    pub exception: Exception,
}

impl TryAssignFrom<senhasegura_rs::ApiError> for ApiError {
    fn assign(&mut self, value: senhasegura_rs::ApiError) -> Result<(), Error> {
        self.response.assign(value.response)?;

        if let Some(exception) = value.exception {
            self.exception.assign(exception)?;
        }

        Ok(())
    }
}

/// Response (i.e. "response") field.
#[repr(C)]
pub struct Response {
    /// HTTP status code.
    pub status: u16,

    /// Response message.
    pub message: *mut c_char,

    /// Flag to indicate whether an error occurred.
    pub error: bool,

    /// Error code.
    pub error_code: i32,
}

impl TryAssignFrom<senhasegura_rs::Response> for Response {
    fn assign(&mut self, value: senhasegura_rs::Response) -> Result<(), Error> {
        self.status = value.status.as_u16();
        assign_str_to_c_char(&value.message, self.message)?;
        self.error = value.error;
        self.error_code = value.error_code;

        Ok(())
    }
}

/// Exception (i.e. "exception") field.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Exception {
    /// Exception code.
    pub code: u16,

    /// Exception message.
    pub message: *mut c_char,

    /// Exception detail (optional).
    pub detail: *mut c_char,
}

impl TryAssignFrom<senhasegura_rs::Exception> for Exception {
    fn assign(&mut self, value: senhasegura_rs::Exception) -> Result<(), Error> {
        self.code = value.code.into();

        assign_str_to_c_char(&value.message, self.message)?;

        if let Some(detail) = value.detail {
            assign_str_to_c_char(&detail, self.detail)?;
        }

        Ok(())
    }
}

pub(crate) fn c_char_to_string(c: *const c_char) -> Result<String, Error> {
    if c.is_null() {
        return Err(Error::invalid_null_pointer());
    }

    let c_str = unsafe { std::ffi::CStr::from_ptr(c) };

    c_str
        .to_str()
        .map(|s| s.to_string())
        .map_err(|_| Error::invalid_string())
}

pub(crate) fn assign_str_to_c_char(src: &str, dst: *mut c_char) -> Result<(), Error> {
    if dst.is_null() {
        return Err(Error::invalid_null_pointer());
    }

    let c_str = std::ffi::CString::new(src).map_err(|_| Error::invalid_string())?;

    unsafe {
        std::ptr::copy_nonoverlapping(c_str.as_ptr(), dst, c_str.as_bytes().len());
    }

    Ok(())
}
