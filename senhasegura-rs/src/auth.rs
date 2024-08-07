use chrono::{DateTime, Duration, Utc};
use oauth2::{AccessToken, EndpointNotSet, EndpointSet, TokenResponse};

use crate::SenhaseguraClient;

pub(super) type OAuth2Client = oauth2::basic::BasicClient<
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Debug)]
pub(super) struct AuthContext {
    access_token: AccessToken,
    expires_at: DateTime<Utc>,
}

impl AuthContext {
    fn new(access_token: AccessToken, expires_at: DateTime<Utc>) -> Self {
        Self {
            access_token,
            expires_at,
        }
    }
}

impl SenhaseguraClient {
    /// Authenticates the client.
    ///
    /// See [OAuth v2.0 authentication](https://docs.senhasegura.io/docs/a2a-how-to-authenticate-an-application#oauth-v20-authentication).
    #[tracing::instrument(level = "trace", skip(self), err)]
    pub(crate) async fn authenticate(&self) -> anyhow::Result<AccessToken> {
        let mut auth_ctx = self.auth_ctx.lock().await;

        match auth_ctx.as_ref() {
            Some(ctx) if ctx.expires_at >= Utc::now() => return Ok(ctx.access_token.clone()),
            _ => {}
        }

        #[cfg(feature = "retry")]
        let http_client = &custom::OAuth2HttpClient::new(&self.http_client);

        #[cfg(not(feature = "retry"))]
        let http_client = &self.http_client;

        let response = self
            .oauth2_client
            .exchange_client_credentials()
            .request_async(http_client)
            .await?;

        let access_token = response.access_token().to_owned();
        let expires_at = Utc::now()
            + response
                .expires_in()
                .and_then(|d| Duration::from_std(d).ok())
                .unwrap_or_default();

        auth_ctx.replace(AuthContext::new(access_token.clone(), expires_at));

        Ok(access_token)
    }
}

#[cfg(feature = "retry")]
mod custom {
    use std::{future::Future, pin::Pin};

    use oauth2::{AsyncHttpClient, HttpClientError, HttpRequest, HttpResponse};

    pub struct OAuth2HttpClient<'a> {
        http_client: &'a reqwest_middleware::ClientWithMiddleware,
    }

    impl<'a> OAuth2HttpClient<'a> {
        pub fn new(http_client: &'a reqwest_middleware::ClientWithMiddleware) -> Self {
            Self { http_client }
        }
    }

    impl<'c> AsyncHttpClient<'c> for OAuth2HttpClient<'_> {
        type Error = HttpClientError<reqwest_middleware::Error>;

        type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Self::Error>> + Send + 'c>>;

        fn call(&'c self, request: HttpRequest) -> Self::Future {
            Box::pin(async move {
                let response = self
                    .http_client
                    .execute(
                        request
                            .try_into()
                            .map_err(|e| Box::new(reqwest_middleware::Error::Reqwest(e)))?,
                    )
                    .await
                    .map_err(Box::new)?;

                let mut builder = http::Response::builder().status(response.status());

                builder = builder.version(response.version());

                for (name, value) in response.headers().iter() {
                    builder = builder.header(name, value);
                }

                builder
                    .body(
                        response
                            .bytes()
                            .await
                            .map_err(|e| Box::new(reqwest_middleware::Error::Reqwest(e)))?
                            .to_vec(),
                    )
                    .map_err(HttpClientError::Http)
            })
        }
    }
}
