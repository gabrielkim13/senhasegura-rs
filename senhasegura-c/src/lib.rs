//! Senhasegura API client C FFI.

#![warn(missing_docs)]

mod pam_core;
pub use pam_core::*;

mod common;
use common::*;

mod error;
use error::*;

use std::{os::raw::c_char, time::Duration};

use url::Url;

/// SenhaSegura API client opaque struct.
pub struct SenhaseguraClient(senhasegura_rs::SenhaseguraClient);

/// Senhasegura API client properties.
#[repr(C)]
pub struct SenhaseguraClientProps {
    /// Base URL of the Senhasegura API.
    pub base_url: *const c_char,

    /// Request timeout, in seconds.
    pub request_timeout: u32,

    /// OAuth2 client credentials ID.
    pub client_id: *const c_char,
    /// OAuth2 client credentials secret.
    pub client_secret: *const c_char,

    /// Base delay of the exponential backoff retry policy, in milliseconds.
    pub base_retry_delay_secs: u32,
    /// Maximum number of retries.
    pub max_n_retries: u32,
}

/// Initializes the Senhasegura API client.
///
/// # Safety
///
/// - The `props` parameter must be a valid pointer to a `SenhaseguraClientProps` struct.
/// - The `*const c_char` fields must be valid C-style strings.
#[no_mangle]
pub unsafe extern "C" fn create_senhasegura_client(
    client: *mut *mut SenhaseguraClient,
    props: *mut SenhaseguraClientProps,
) -> ErrorCode {
    let props = match unsafe { props.as_ref() } {
        Some(props) => props,
        None => return ErrorCode::InvalidNullPointer,
    };

    match _create_senhasegura_client(props) {
        Ok(c) => {
            *client = Box::into_raw(Box::new(c));

            ErrorCode::Ok
        }
        Err(e) => e.into(),
    }
}

fn _create_senhasegura_client(props: &SenhaseguraClientProps) -> Result<SenhaseguraClient, Error> {
    let base_url = {
        let base_url = c_char_to_string(props.base_url)?;

        Url::parse(&base_url).map_err(|_| Error::invalid_url())?
    };

    let client_id = c_char_to_string(props.client_id)?;
    let client_secret = c_char_to_string(props.client_secret)?;

    let rs_client = senhasegura_rs::SenhaseguraClient::builder(base_url, client_id, client_secret)
        .request_timeout(Duration::from_secs(props.request_timeout as u64))
        .base_retry_delay_secs(props.base_retry_delay_secs)
        .max_n_retries(props.max_n_retries)
        .build()
        .unwrap();

    Ok(SenhaseguraClient(rs_client))
}

/// Deinitializes the Senhasegura API client.
///
/// # Safety
///
/// - The `client` parameter must be a valid pointer to a `SenhaseguraClient` struct.
#[no_mangle]
pub unsafe extern "C" fn destroy_senhasegura_client(client: *mut SenhaseguraClient) {
    drop(unsafe { Box::from_raw(client) });
}
