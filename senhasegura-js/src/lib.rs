#[macro_use]
extern crate napi_derive;

mod pam_core;

use url::Url;

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
    pub max_retries: Option<u32>,
}

/// Senhasegura API client.
#[napi]
pub struct SenhaseguraClient {
    client: senhasegura_rs::SenhaseguraClient,
}

#[napi]
impl SenhaseguraClient {
    /// Creates a new Senhasegura API client.
    #[napi(factory)]
    pub fn create(props: SenhaseguraClientProps) -> napi::Result<Self> {
        let base_url = Url::parse(&props.base_url).map_err(Self::map_url_parse_error)?;

        let mut builder = senhasegura_rs::SenhaseguraClientBuilder::new(
            base_url,
            props.client_id,
            props.client_secret,
        );

        if let Some(request_timeout) = props.request_timeout {
            builder =
                builder.request_timeout(std::time::Duration::from_secs(request_timeout as u64));
        }

        if let Some(base_retry_delay) = props.base_retry_delay {
            builder =
                builder.base_retry_delay(std::time::Duration::from_secs(base_retry_delay as u64));
        }

        if let Some(max_retries) = props.max_retries {
            builder = builder.max_retries(max_retries as usize);
        }

        Ok(Self {
            client: builder.build().map_err(Self::map_url_parse_error)?,
        })
    }

    fn map_url_parse_error(error: url::ParseError) -> napi::Error {
        napi::Error::from_reason(error.to_string())
    }
}
