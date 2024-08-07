use http::{Method, StatusCode};
use serde_json::json;
use test_context::test_context;
use wiremock::{
    matchers::{body_json, header, method, path},
    Mock, ResponseTemplate,
};

use senhasegura_rs::{CreateProtectedInformationApi, CreateProtectedInformationApiRequest};

use crate::fixture::Fixture;

#[test_context(Fixture)]
#[tokio::test]
async fn test_create_protected_information(fixture: &mut Fixture) {
    Mock::given(method(Method::POST))
        .and(path("/iso/pam/info"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "name": "saas_vault1",
            "content":"login: mt4adm, password: mt4admp4ss",
            "identifier": "INFOSAASVAULT1",
            "type": "access Credential"
        })))
        .respond_with(
            ResponseTemplate::new(StatusCode::CREATED).set_body_json(json!({
                "response": {
                    "status": 201,
                    "mensagem": "Information successfully registered!",
                    "erro": false,
                    "message": "Information successfully registered!",
                    "error": false
                },
                "info": {
                    "name": "saas_vault1",
                    "type": "access credential",
                    "service": "saas_client",
                    "url": "10.10.10.2",
                    "content": "login: mt4adm, password: mt4admp4ss",
                    "users_allowed": "admin, account_manager, mscharra",
                    "identifier": "INFOSAASVAULT1"
                }
            })),
        )
        .expect(1)
        .mount(fixture.server())
        .await;

    let response = fixture
        .client()
        .create_protected_information(CreateProtectedInformationApiRequest {
            name: Some("saas_vault1".to_string()),
            content: "login: mt4adm, password: mt4admp4ss".to_string(),
            identifier: Some("INFOSAASVAULT1".to_string()),
            r#type: Some("access Credential".to_string()),
        })
        .await
        .unwrap();

    assert_eq!(response.info.identifier.unwrap(), "INFOSAASVAULT1");
    assert_eq!(response.info.content, "login: mt4adm, password: mt4admp4ss",);
}
