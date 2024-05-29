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