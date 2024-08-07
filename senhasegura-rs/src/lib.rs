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
//! # tokio_test::block_on(async {
//! let base_url = "https://senhasegura.acme.com".parse()?;
//! let client_id = "client_id";
//! let client_secret = "client_secret";
//!
//! let client = SenhaseguraClient::builder(base_url, client_id, client_secret).build()?;
//!
//! // Access protected information
//! println!("{:#?}", client.access_protected_information(28).await?);
//!
//! # Ok::<_, anyhow::Error>(())
//! # });
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

use std::sync::Arc;

use tokio::sync::Mutex;
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

    #[cfg(feature = "retry")]
    http_client: reqwest_middleware::ClientWithMiddleware,
    #[cfg(not(feature = "retry"))]
    http_client: reqwest::Client,

    oauth2_client: OAuth2Client,
    auth_ctx: Arc<Mutex<Option<AuthContext>>>,
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

    /// Performs an authenticated API request, returning a normalized result.
    #[tracing::instrument(level = "trace", skip(self), err)]
    async fn do_api_request<
        TPath: AsRef<str> + std::fmt::Debug,
        TReq: serde::ser::Serialize + std::fmt::Debug,
        TRes: serde::de::DeserializeOwned + std::fmt::Debug,
    >(
        &self,
        method: http::method::Method,
        path: TPath,
        data: Option<TReq>,
    ) -> Result<TRes, Error> {
        let access_token = self.authenticate().await?;

        let url = self.base_url.join(path.as_ref())?;

        let mut req = self
            .http_client
            .request(method, url)
            .bearer_auth(access_token.secret());

        if let Some(data) = data {
            req = req.json(&data);
        }

        let response = req.send().await?;

        if response.status().is_success() {
            let response = response.json().await?;

            Ok(response)
        } else {
            let api_error = response.json().await?;

            Err(Error::Api(api_error))
        }
    }
}

/// Senhasegura API client builder.
pub struct SenhaseguraClientBuilder {
    base_url: Url,

    request_timeout: Option<std::time::Duration>,

    client_id: String,
    client_secret: String,

    #[cfg(feature = "retry")]
    base_retry_delay_secs: Option<u32>,
    #[cfg(feature = "retry")]
    max_n_retries: Option<u32>,
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
            base_retry_delay_secs: None,
            #[cfg(feature = "retry")]
            max_n_retries: None,
        }
    }

    /// Sets the request timeout.
    pub fn request_timeout(mut self, request_timeout: std::time::Duration) -> Self {
        self.request_timeout = Some(request_timeout);
        self
    }

    /// Sets the base retry delay, in seconds.
    #[cfg(feature = "retry")]
    pub fn base_retry_delay_secs(mut self, base_retry_delay_secs: u32) -> Self {
        self.base_retry_delay_secs = Some(base_retry_delay_secs);
        self
    }

    /// Sets the maximum number of retries.
    #[cfg(feature = "retry")]
    pub fn max_n_retries(mut self, max_n_retries: u32) -> Self {
        self.max_n_retries = Some(max_n_retries);
        self
    }

    /// Disables request retries.
    #[cfg(feature = "retry")]
    pub fn disable_retries(self) -> Self {
        self.base_retry_delay_secs(0).max_n_retries(0)
    }

    /// Builds the Senhasegura API client.
    pub fn build(self) -> Result<SenhaseguraClient, Error> {
        let base_url = {
            let mut base_url = self.base_url;

            let mut path = base_url.path().to_string();
            if !path.ends_with('/') {
                path.push('/');
            }
            base_url.set_path(&path);

            base_url
        };

        let http_client = reqwest::Client::builder()
            .use_rustls_tls()
            .timeout(
                self.request_timeout
                    .unwrap_or(std::time::Duration::from_secs(10)),
            )
            .build()?;

        #[cfg(feature = "retry")]
        let http_client = {
            use reqwest_retry::{policies::ExponentialBackoff, Jitter, RetryTransientMiddleware};

            let retry_policy = ExponentialBackoff::builder()
                .jitter(Jitter::Full)
                .base(self.base_retry_delay_secs.unwrap_or(2))
                .build_with_max_retries(self.max_n_retries.unwrap_or(3));

            reqwest_middleware::ClientBuilder::new(http_client)
                .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build()
        };

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
        })
    }
}
