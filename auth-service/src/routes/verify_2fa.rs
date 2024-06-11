use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
};

#[derive(Debug, Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}

pub async fn verify_2fa(
    Json(request): Json<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    
    let email = match Email::parse(request.email.clone()) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials)
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id.clone()) {
        Ok(login_attempt_id) => login_attempt_id,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(two_fa_code) => two_fa_code,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    
    Ok(StatusCode::OK.into_response())

}