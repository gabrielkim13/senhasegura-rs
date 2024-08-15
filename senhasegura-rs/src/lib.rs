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

#![allow(clippy::blocks_in_conditions)] // For `async-trait`

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
#[derive(Debug)]
#[cfg_attr(feature = "napi", napi_derive::napi)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
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

#[cfg(feature = "blocking")]
impl SenhaseguraClient {
    pub(crate) fn async_runtime(&self) -> Result<tokio::runtime::Handle, Error> {
        use once_cell::sync::OnceCell;

        static RUNTIME: OnceCell<tokio::runtime::Runtime> = OnceCell::new();

        // This shouldn't really happen in real-world scenarios, but we might enable the `uniffi`
        // feature during tests, which will already have its own async runtime.
        //
        // In this case, we need to return a handle to the current runtime instead of creating a new
        // one.
        let handle = tokio::runtime::Handle::try_current().or_else(|_| {
            RUNTIME
                .get_or_try_init(|| {
                    tokio::runtime::Builder::new_multi_thread()
                        .enable_all()
                        .build()
                })
                .map(|r| r.handle().clone())
        })?;

        Ok(handle)
    }
}

#[cfg(feature = "napi")]
mod senhasegura_js {
    use super::*;

    use napi_derive::napi;

    /// Senhasegura API client options.
    #[napi(object)]
    pub struct SenhaseguraClientProps {
        /// Base URL of the Senhasegura API.
        pub base_url: String,

        /// Request timeout, in seconds.
        pub request_timeout: Option<u32>,

        /// OAuth2 client credentials ID.
        pub client_id: String,
        /// OAuth2 client credentials secret.
        pub client_secret: String,

        /// Base delay of the exponential backoff retry policy, in milliseconds.
        pub base_retry_delay: Option<u32>,
        /// Maximum number of retries.
        pub max_n_retries: Option<u32>,
    }

    #[napi]
    impl SenhaseguraClient {
        /// Creates a new Senhasegura API client.
        #[napi(factory)]
        pub fn create(props: SenhaseguraClientProps) -> Result<Self, Error> {
            let base_url = Url::parse(&props.base_url)?;

            let mut builder =
                SenhaseguraClientBuilder::new(base_url, props.client_id, props.client_secret);

            if let Some(request_timeout) = props.request_timeout {
                builder =
                    builder.request_timeout(std::time::Duration::from_secs(request_timeout as u64));
            }

            if let Some(base_retry_delay_secs) = props.base_retry_delay {
                builder = builder.base_retry_delay_secs(base_retry_delay_secs);
            }

            if let Some(max_n_retries) = props.max_n_retries {
                builder = builder.max_n_retries(max_n_retries);
            }

            builder.build()
        }
    }
}

#[cfg(feature = "uniffi")]
mod senhasegura_uniffi {
    use super::*;

    uniffi::setup_scaffolding!("senhasegura");

    #[uniffi::export]
    impl SenhaseguraClient {
        /// Creates a new Senhasegura API client.
        #[uniffi::constructor]
        fn new(
            base_url: String,
            client_id: String,
            client_secret: String,
        ) -> Result<Arc<SenhaseguraClient>, Error> {
            let client = SenhaseguraClientBuilder::new(base_url.parse()?, client_id, client_secret)
                .build()?;

            Ok(Arc::new(client))
        }
    }
}

#[cfg(feature = "uniffi")]
pub use senhasegura_uniffi::*;
