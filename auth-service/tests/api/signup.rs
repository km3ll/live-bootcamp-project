use crate::helpers::{get_random_email, TestApp};
use serde_json::Value;
use auth_service::routes::SignupResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    
    // Given
    let app: TestApp = TestApp::new().await;
    let random_email: String = get_random_email();

    let test_cases: [Value; 2] = [
        serde_json::json!(
            {
                "password": "pass1234",
                "requires2FA": true
            }
        ),
        serde_json::json!(
            {
                "email": "johnwick@example.com",
                "password": "****",
            }
        )
    ];

    // When-Then
    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }

}

#[tokio::test]
async fn should_return_201_if_valid_input() {

    // Given
    let app: TestApp = TestApp::new().await;
    let random_email: String = get_random_email();
    let body: Value = serde_json::json!({
        "email": random_email,
        "password": "******",
        "requires2FA": true
    });
    // When
    let response = app.post_signup(&body).await;
    // Then
    assert_eq!(response.status().as_u16(), 201);
    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );

}


#[tokio::test]
async fn should_return_400_if_invalid_input() {

    // Given
    let app: TestApp = TestApp::new().await;
    let body: Value = serde_json::json!({
        "email": "not-an-email",
        "password": "******",
        "requires2FA": true
    });
    // When
    let response = app.post_signup(&body).await;
    // Then
    assert_eq!(response.status().as_u16(), 400);

}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {

    // Given
    let app: TestApp = TestApp::new().await;
    let body: Value = serde_json::json!({
        "email": "johnwick@gmail.com",
        "password": "******",
        "requires2FA": true
    });
    let response1 = app.post_signup(&body).await;
    // When
    let response2 = app.post_signup(&body).await;
    // Then
    assert_eq!(response2.status().as_u16(), 409);

}