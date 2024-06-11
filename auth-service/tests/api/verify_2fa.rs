use auth_service::{
    domain::{LoginAttemptId, TwoFACode},
    ErrorResponse,
};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let login_attempt_id = LoginAttemptId::default().as_ref().to_owned();
    let two_fa_code = TwoFACode::default().as_ref().to_owned();

    let test_cases = vec![
        (
            "invalid_email",
            login_attempt_id.as_str(),
            two_fa_code.as_str(),
        ),
        (
            random_email.as_str(),
            "invalid_login_attempt_id",
            two_fa_code.as_str(),
        ),
        (
            random_email.as_str(),
            login_attempt_id.as_str(),
            "invalid_two_fa_code",
        ),
        ("", "", ""),
    ];

    for (email, login_attempt_id, code) in test_cases {
        let request_body = serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        });

        let response = app.post_verify_2fa(&request_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            request_body
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let login_attempt_id = LoginAttemptId::default().as_ref().to_owned();

    let test_cases = [
        serde_json::json!({
            "2FACode": "123456",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "loginAttemptId": login_attempt_id,
        }),
        serde_json::json!({
            "2FACode": "123456",
            "email": random_email,
        }),
        serde_json::json!({
            "2FACode": "123456",
            "loginAttemptId": login_attempt_id,
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": login_attempt_id,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}