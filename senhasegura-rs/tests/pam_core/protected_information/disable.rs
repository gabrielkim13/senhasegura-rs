use http::{Method, StatusCode};
use serde_json::json;
use test_context::test_context;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use senhasegura_rs::DisableProtectedInformationApi;

use crate::fixture::Fixture;

#[test_context(Fixture)]
#[tokio::test]
async fn test_disable_protected_information(fixture: &mut Fixture) {
    let id = "1".to_string();

    Mock::given(method(Method::DELETE))
        .and(path(format!("/iso/pam/info/{id}")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(json!({
            "response": {
                "status": 200,
                "mensagem": "Information successfully disabled",
                "erro": false,
                "message": "Information successfully disabled",
                "error": false
            }
        })))
        .expect(1)
        .mount(fixture.server())
        .await;

    fixture
        .client()
        .disable_protected_information(id)
        .await
        .unwrap();
}
