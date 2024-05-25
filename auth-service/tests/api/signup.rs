use crate::helpers::{get_random_email, TestApp};
use serde_json::Value;

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
        println!("Request: {:?}", test_case);
        let response = app.post_signup(test_case).await;
        println!("Response: {:?}", response);
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

    let response = app.post_signup(&body).await;
    println!("{:?}", response);
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed for input: {:?}",
        body
    );

}