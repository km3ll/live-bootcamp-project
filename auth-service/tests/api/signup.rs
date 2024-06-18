use crate::helpers::{get_random_email, TestApp};
use serde_json::Value;
use auth_service::{routes::SignupResponse, ErrorResponse};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    
    // Given
    let mut app: TestApp = TestApp::new().await;
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

    app.clean_up().await;

}

#[tokio::test]
async fn should_return_201_if_valid_input() {

    // Given
    let mut app: TestApp = TestApp::new().await;
    let random_email: String = get_random_email();
    let body: Value = serde_json::json!({
        "email": random_email,
        "password": "********",
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

    app.clean_up().await;

}


#[tokio::test]
async fn should_return_400_if_invalid_input() {

    // Given
    let mut app: TestApp = TestApp::new().await;

    let test_cases: [Value; 3] = [
        // empty email
        serde_json::json!( { "email": "", "password": "********", "requires2FA": true } ),
        // email without '@'
        serde_json::json!( { "email": "not-an-email", "password": "********", "requires2FA": true } ),
        // password.length less than 8 characters
        serde_json::json!( { "email": "user@gmail.com", "password": "123", "requires2FA": false } )
    ];

    for test_body in test_cases.iter() {
        // When
        let response = app.post_signup(test_body).await;
        //Then
        assert_eq!(response.status().as_u16(), 400, "Failed for input {:?}", test_body);
        assert_eq!(
            response.json::<ErrorResponse>().await.expect("Could not deserialize response body to ErrorResponse").error,
            "Invalid credentials".to_owned()
        );
    }

    app.clean_up().await;

}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {

    // Given
    let mut app: TestApp = TestApp::new().await;
    let body = serde_json::json!(
        {
            "email": "user@gmail.com",
            "password": "87654321",
            "requires2FA": true
        }
    );

    // When-Then
    let response1 = app.post_signup(&body).await;
    assert_eq!(response1.status().as_u16(), 201, "Failed for input {:?}", body);

    // When-Then
    let response2 = app.post_signup(&body).await;
    assert_eq!(
        response2.json::<ErrorResponse>().await.expect("Could not deserialize response body to ErrorResponse").error,
        "User already exists".to_owned()
    );

    app.clean_up().await;

}