use std::sync::Once;

use http::{Method, StatusCode};
use serde_json::json;
use test_context::AsyncTestContext;
use url::Url;
use wiremock::{
    matchers::{body_string, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

use senhasegura_rs::SenhaseguraClient;

pub struct Fixture {
    server: MockServer,
    client: SenhaseguraClient,
}

impl Fixture {
    pub const CLIENT_ID: &'static str = "client_id";
    pub const CLIENT_SECRET: &'static str = "client_secret";

    pub fn server(&self) -> &MockServer {
        &self.server
    }

    pub fn client(&self) -> &SenhaseguraClient {
        &self.client
    }

    fn init_log() {
        static LOG: Once = Once::new();

        LOG.call_once(|| {
            let subscriber = tracing_subscriber::FmtSubscriber::new();
            tracing::subscriber::set_global_default(subscriber).unwrap();
        })
    }
}

impl AsyncTestContext for Fixture {
    async fn setup() -> Self {
        Self::init_log();

        let server = MockServer::start().await;

        let client = {
            let base_url = Url::parse(&server.uri()).unwrap();

            SenhaseguraClient::builder(base_url, Self::CLIENT_ID, Self::CLIENT_SECRET)
                .build()
                .unwrap()
        };

        // OAuth2 client credentials authentication mock.
        //
        // This should happen at most once for every test case, since the access token is meant to
        // be cached by the client.
        Mock::given(method(Method::POST))
            .and(path("/iso/oauth2/token"))
            .and(header("content-type", "application/x-www-form-urlencoded"))
            .and(body_string(format!(
                "grant_type=client_credentials&client_id={}&client_secret={}",
                Self::CLIENT_ID,
                Self::CLIENT_SECRET
            )))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(json!({
                "access_token": "access_token",
                "token_type": "bearer",
                "expires_in": 3600
            })))
            .up_to_n_times(1)
            .mount(&server)
            .await;

        Self { client, server }
    }
}
