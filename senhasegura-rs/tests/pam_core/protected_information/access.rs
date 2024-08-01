use http::{Method, StatusCode};
use serde_json::json;
use test_context::test_context;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use senhasegura_rs::AccessProtectedInformationAPI;

use crate::fixture::Fixture;

#[test_context(Fixture)]
#[tokio::test]
async fn test_access_protected_information(fixture: &mut Fixture) {
    let id = 28;

    Mock::given(method(Method::GET))
        .and(path(format!("/iso/pam/info/{id}")))
        .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(json!({
            "response": {
                "status": 200,
                "message": "Information 28",
                "error": false,
                "error_code": 0,
                "detail": "",
                "mensagem": "Information 28",
                "erro": false,
                "cod_erro": 0
            },
            "info": {
                "id": "28",
                "tag": null,
                "type": "Access credential",
                "content": "hdjskasdhdj2789208/3\\G+H-J_K'#JK\"NAOAPARECE\"JSJSJSJS"
            }
        })))
        .expect(1)
        .mount(fixture.server())
        .await;

    let response = fixture.client().access_protected_information(id).unwrap();

    assert_eq!(response.info.id, id);
    assert_eq!(
        response.info.content,
        "hdjskasdhdj2789208/3\\G+H-J_K'#JK\"NAOAPARECE\"JSJSJSJS"
    );
}
