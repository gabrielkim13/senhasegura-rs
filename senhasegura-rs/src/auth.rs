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
    pub(crate) fn authenticate(&self) -> anyhow::Result<AccessToken> {
        let mut auth_ctx = self.auth_ctx.lock().unwrap();

        match auth_ctx.as_ref() {
            Some(ctx) if ctx.expires_at >= Utc::now() => return Ok(ctx.access_token.clone()),
            _ => {}
        }

        let response = self
            .oauth2_client
            .exchange_client_credentials()
            .request(&self.http_client)?;

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
