use secrecy::Secret;
use serde_json::Value;
use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email,
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
};


#[tokio::test]
async fn should_return_422_if_malformed_credentials() {

    // Given
    let mut app: TestApp = TestApp::new().await;

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

    app.clean_up().await;

}

#[tokio::test]
async fn should_return_400_if_invalid_input() {

    // Given
    let mut app: TestApp = TestApp::new().await;

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

    app.clean_up().await;

}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {

    // Given
    let mut app: TestApp = TestApp::new().await;
    let body = serde_json::json!(
        {
            "email": "user@gmail.com",
            "password": "87654321"
        }
    );

    // When-Then
    let response = app.post_login(&body).await;
    assert_eq!(response.status().as_u16(), 401, "Failed for input {:?}", body);

    app.clean_up().await;   

}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {

    // Given
    let mut app: TestApp = TestApp::new().await;
    let random_email: String = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password1100",
        "requires2FA": false
    });

    // Signup
    // When
    let response = app.post_signup(&signup_body).await;
    // Then
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input {:?}",
        signup_body
    );

    // Login
    // When
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password1100"
    });

    // Then
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    app.clean_up().await;

}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {

    // Given
    let mut app: TestApp = TestApp::new().await;
    let random_email: String = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password1100",
        "requires2FA": true
    });

    // Signup
    // When
    let response = app.post_signup(&signup_body).await;
    // Then
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input {:?}",
        signup_body
    );

    // Login
    // When
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password1100"
    });

    // Then
    let response = app.post_login(&login_body).await;
    assert_eq!(
        response.status().as_u16(), 
        206,
        "The API did not return a 206 PARTIAL CONTENT"
    );

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());
    
    app.clean_up().await;

    let two_fa_code_store = app.two_fa_code_store.read().await;

    let code_tuple = two_fa_code_store
        .get_code(&Email::parse(Secret::new(random_email)).unwrap())
        .await
        .expect("Faile d to get 2FA code");

    assert_eq!(code_tuple.0.as_ref(), json_body.login_attempt_id);

}