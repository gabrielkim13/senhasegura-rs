use http::{Method, StatusCode};
use serde_json::json;
use test_context::test_context;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use senhasegura_rs::{
    AccessProtectedInformationApi, Error, ExceptionCode, PAMCoreExceptionCode,
    ProtectedInformationExceptionCode,
};

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

#[test_context(Fixture)]
#[tokio::test]
async fn test_access_protected_information_not_found(fixture: &mut Fixture) {
    let id = 28;

    Mock::given(method(Method::GET))
        .and(path(format!("/iso/pam/info/{id}")))
        .respond_with(
            ResponseTemplate::new(StatusCode::BAD_REQUEST).set_body_json(json!({
                "response": {
                    "status": 400,
                    "mensagem": "1023: Information not found",
                    "erro": true,
                    "message": "1023: Information not found",
                    "error": true
                },
                "exception": {
                    "code": 1023,
                    "message": "1023: Information not found",
                    "detail": null
                }
            })),
        )
        .expect(1)
        .mount(fixture.server())
        .await;

    let response = fixture
        .client()
        .access_protected_information(id)
        .unwrap_err();

    if let Error::Api(api_error) = response {
        let response = api_error.response;

        assert_eq!(response.status, StatusCode::BAD_REQUEST);
        assert_eq!(response.message, "1023: Information not found");
        assert!(response.error);

        let exception = api_error.exception.unwrap();

        assert!(matches!(
            exception.code,
            ExceptionCode::PAMCore(PAMCoreExceptionCode::ProtectedInformation(
                ProtectedInformationExceptionCode::InformationNotFound
            ))
        ));
        assert_eq!(exception.message, "1023: Information not found");
        assert!(exception.detail.is_none());
    } else {
        panic!("Unexpected error: {:?}", response);
    }
}

#[test_context(Fixture)]
#[tokio::test]
async fn test_access_protected_information_unknown_exception(fixture: &mut Fixture) {
    let id = 28;

    Mock::given(method(Method::GET))
        .and(path(format!("/iso/pam/info/{id}")))
        .respond_with(
            ResponseTemplate::new(StatusCode::BAD_REQUEST).set_body_json(json!({
                "response": {
                    "status": 400,
                    "mensagem": "9999: Unknown exception",
                    "erro": true,
                    "message": "9999: Unknown exception",
                    "error": true
                },
                "exception": {
                    "code": 9999,
                    "message": "9999: Unknown exception",
                    "detail": null
                }
            })),
        )
        .expect(1)
        .mount(fixture.server())
        .await;

    let response = fixture
        .client()
        .access_protected_information(id)
        .unwrap_err();

    if let Error::Api(api_error) = response {
        let exception = api_error.exception.unwrap();

        assert!(matches!(exception.code, ExceptionCode::Unknown(9999)));
    } else {
        panic!("Unexpected error: {:?}", response);
    }
}
