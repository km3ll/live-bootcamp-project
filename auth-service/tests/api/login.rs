use serde_json::Value;
use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {

    // Given
    let app: TestApp = TestApp::new().await;

    let test_cases: [Value; 2] = [
        serde_json::json!({
            "name": "",
            "lastname": "********"
        }),
        serde_json::json!({
            "e-mail": "user@gmail.com",
            "pass-code": "**"
        }),
    ];

    for test_body in test_cases.iter() {
        // When
        let response = app.post_login(test_body).await;
        //Then
        assert_eq!(response.status().as_u16(), 422, "Failed for input {:?}", test_body);
    }

}

#[tokio::test]
async fn should_return_400_if_invalid_input() {

    // Given
    let app: TestApp = TestApp::new().await;

    let test_cases: [Value; 3] = [
        serde_json::json!({
            "email": "",
            "password": "********"
        }),
        serde_json::json!({
            "email": "gmail.com",
            "password": "********"
        }),
        serde_json::json!({
            "email": "use@gmail.com",
            "password": "123"
        }),
    ];

    for test_body in test_cases.iter() {
        // When
        let response = app.post_login(test_body).await;
        //Then
        assert_eq!(response.status().as_u16(), 400, "Failed for input {:?}", test_body);
    }

}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {

    // Given
    let app: TestApp = TestApp::new().await;
    let body = serde_json::json!(
        {
            "email": "user@gmail.com",
            "password": "87654321"
        }
    );

    // When-Then
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 401, "Failed for input {:?}", body);

}