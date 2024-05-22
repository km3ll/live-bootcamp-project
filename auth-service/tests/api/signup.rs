use crate::helpers::{get_random_email, TestApp};
use serde_json::Value;

#[tokio::test]
async fn signup_should_return_422_if_malformed_input() {
    
    // Given
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases: [Value; 4] = [
        serde_json::json!({
            "password": "pass1234",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": "johnwick@example.com",
            "password": "****",
        }),
        serde_json::json!({
            "email": "johnwick@example.com",
            "requires2FA": null,
        }),
        serde_json::json!({
            "name": "john",
            "lastname": "wick",
        }),
    ];

    // When-Then
    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

}