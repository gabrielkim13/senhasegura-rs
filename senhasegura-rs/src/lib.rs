//! # senhasegura-rs
//!
//! Senhasegura API client for Rust.
//!
//! See [A2A - APIs](https://docs.senhasegura.io/docs/a2a-apis).
//!
//! # Example
//!
//! ```no_run
//! use senhasegura_rs::{AccessProtectedInformationApi, SenhaseguraClient};
//!
//! # fn main() -> anyhow::Result<()> {
//! let base_url = "https://senhasegura.acme.com".parse()?;
//! let client_id = "client_id";
//! let client_secret = "client_secret";
//!
//! let client = SenhaseguraClient::builder(base_url, client_id, client_secret).build()?;
//!
//! // Access protected information
//! println!("{:#?}", client.access_protected_information(28)?);
//! #
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

mod auth;
use auth::*;

mod common;
pub use common::*;

mod error;
pub use error::*;

/// PAM Core APIs.
///
/// See [PAM Core APIs](https://docs.senhasegura.io/docs/a2a-apis-pam-core).
pub mod pam_core;
pub use pam_core::*;

use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use url::Url;

/// Trait to interact with Senhasegura APIs.
///
/// See [A2A - APIs](https://docs.senhasegura.io/docs/a2a-apis).
pub trait SenhaseguraApi: PAMCoreApi {}

impl<T> SenhaseguraApi for T where T: PAMCoreApi {}

/// Senhasegura API client.
#[derive(Clone, Debug)]
pub struct SenhaseguraClient {
    base_url: Url,

    http_client: ureq::Agent,

    oauth2_client: OAuth2Client,
    auth_ctx: Arc<Mutex<Option<AuthContext>>>,

    #[cfg(feature = "retry")]
    base_retry_delay: std::time::Duration,
    #[cfg(feature = "retry")]
    max_retries: usize,
}

impl SenhaseguraClient {
    /// Creates a new Senhasegura API client builder.
    pub fn builder(
        base_url: Url,
        client_id: impl ToString,
        client_secret: impl ToString,
    ) -> SenhaseguraClientBuilder {
        SenhaseguraClientBuilder::new(base_url, client_id, client_secret)
    }

    /// Performs an API operation, according to the client's settings.
    ///
    /// See the `retry` feature for more information.
    #[cfg(feature = "retry")]
    #[tracing::instrument(level = "trace", skip(self), err)]
    fn do_api_operation<
        TReq: serde::ser::Serialize + std::fmt::Debug + Clone,
        TRes: serde::de::DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        method: http::method::Method,
        path: &str,
        data: Option<TReq>,
    ) -> Result<TRes, Error> {
        self.with_retry(|| self.do_api_request(method.clone(), path, data.clone()))
    }

    /// Performs an API operation, according to the client's settings.
    #[cfg(not(feature = "retry"))]
    #[tracing::instrument(level = "trace", skip(self), err)]
    fn do_api_operation<
        TReq: serde::ser::Serialize + std::fmt::Debug,
        TRes: serde::de::DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        method: http::method::Method,
        path: &str,
        data: Option<TReq>,
    ) -> Result<TRes, Error> {
        self.do_api_request(method, path, data)
    }

    /// Performs an authenticated API request, returning a normalized result.
    #[tracing::instrument(level = "trace", skip(self), err)]
    fn do_api_request<
        TReq: serde::ser::Serialize + std::fmt::Debug,
        TRes: serde::de::DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        method: http::method::Method,
        path: &str,
        data: Option<TReq>,
    ) -> Result<TRes, Error> {
        let access_token = self.authenticate()?;

        let result = {
            use http::method::Method;

            let url = self.base_url.join(path)?;
            let path = url.as_str();

            let req = match method {
                Method::GET => self.http_client.get(path),
                Method::POST => self.http_client.post(path),
                Method::PUT => self.http_client.put(path),
                Method::DELETE => self.http_client.delete(path),
                Method::HEAD => self.http_client.head(path),
                Method::PATCH => self.http_client.patch(path),
                _ => return Err(Error::Other(anyhow!("Unsupported HTTP method"))),
            }
            .set(
                http::header::AUTHORIZATION.as_str(),
                &format!("Bearer {}", access_token.secret()),
            );

            match data {
                Some(data) => req.send_json(data),
                None => req.call(),
            }
        };

        match result {
            Ok(res) => {
                let response = res.into_json()?;

                Ok(response)
            }
            Err(ureq::Error::Status(_, res)) => {
                let api_error = res.into_json::<crate::ApiError>()?;

                Err(Error::Api(api_error))
            }
            Err(ureq::Error::Transport(transport)) => Err(Error::Transport(Box::new(transport))),
        }
    }

    /// Performs an API operation using the preconfigured retry policy.
    #[cfg(feature = "retry")]
    #[tracing::instrument(level = "trace", skip_all, err)]
    fn with_retry<T: std::fmt::Debug, O: FnMut() -> Result<T, Error>>(
        &self,
        mut operation: O,
    ) -> Result<T, Error> {
        use retry::{
            delay::{jitter, Exponential},
            retry_with_index, OperationResult,
        };

        retry_with_index(
            Exponential::from_millis(self.base_retry_delay.as_millis() as u64)
                .map(jitter)
                .take(self.max_retries),
            |i| match operation() {
                Ok(res) => OperationResult::Ok(res),
                Err(err @ (Error::Api(_) | Error::Other(_))) => {
                    tracing::debug!(error = ?err, "Unrecoverable error");

                    OperationResult::Err(err)
                }
                Err(err @ Error::Transport(_)) => {
                    tracing::debug!(error = ?err, i = i + 1, "Recoverable error; attempting retry");

                    OperationResult::Retry(err)
                }
            },
        )
        .map_err(|e| e.error)
    }
}

/// Senhasegura API client builder.
pub struct SenhaseguraClientBuilder {
    base_url: Url,

    request_timeout: Option<std::time::Duration>,

    client_id: String,
    client_secret: String,

    #[cfg(feature = "retry")]
    base_retry_delay: Option<std::time::Duration>,
    #[cfg(feature = "retry")]
    max_retries: Option<usize>,
}

impl SenhaseguraClientBuilder {
    /// Creates a new Senhasegura API client builder.
    pub fn new(base_url: Url, client_id: impl ToString, client_secret: impl ToString) -> Self {
        Self {
            base_url,

            request_timeout: None,

            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),

            #[cfg(feature = "retry")]
            base_retry_delay: None,
            #[cfg(feature = "retry")]
            max_retries: None,
        }
    }

    /// Sets the request timeout.
    pub fn request_timeout(mut self, request_timeout: std::time::Duration) -> Self {
        self.request_timeout = Some(request_timeout);
        self
    }

    /// Sets the base retry delay.
    #[cfg(feature = "retry")]
    pub fn base_retry_delay(mut self, base_retry_delay: std::time::Duration) -> Self {
        self.base_retry_delay = Some(base_retry_delay);
        self
    }

    /// Sets the maximum number of retries.
    #[cfg(feature = "retry")]
    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Disables request retries.
    #[cfg(feature = "retry")]
    pub fn disable_retries(self) -> Self {
        self.base_retry_delay(std::time::Duration::ZERO)
            .max_retries(0)
    }

    /// Builds the Senhasegura API client.
    pub fn build(self) -> Result<SenhaseguraClient, url::ParseError> {
        let base_url = {
            let mut base_url = self.base_url;

            let mut path = base_url.path().to_string();
            if !path.ends_with('/') {
                path.push('/');
            }
            base_url.set_path(&path);

            base_url
        };

        let http_client = ureq::AgentBuilder::new()
            .timeout(
                self.request_timeout
                    .unwrap_or(std::time::Duration::from_secs(10)),
            )
            .build();

        let oauth2_client = oauth2::basic::BasicClient::new(oauth2::ClientId::new(self.client_id))
            .set_client_secret(oauth2::ClientSecret::new(self.client_secret))
            .set_token_uri(oauth2::TokenUrl::from_url(
                base_url.join("iso/oauth2/token")?,
            ))
            .set_auth_type(oauth2::AuthType::RequestBody);

        Ok(SenhaseguraClient {
            base_url,

            http_client,

            oauth2_client,
            auth_ctx: Default::default(),

            #[cfg(feature = "retry")]
            base_retry_delay: self
                .base_retry_delay
                .unwrap_or(std::time::Duration::from_secs(2)),
            #[cfg(feature = "retry")]
            max_retries: self.max_retries.unwrap_or(3),
        })
    }
}
