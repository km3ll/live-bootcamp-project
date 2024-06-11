use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{
    app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode}
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
    State(state): State<AppState>,
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

    let two_fa_code_store = state.two_fa_code_store.write().await;

    let code_tuple = match two_fa_code_store.get_code(&email).await {
        Ok(code_tuple) => code_tuple,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };
    
    if !code_tuple.0.eq(&login_attempt_id) || !code_tuple.1.eq(&two_fa_code) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    Ok(StatusCode::OK.into_response())

}