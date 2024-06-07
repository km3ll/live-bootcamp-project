use axum::{
    extract::State,
    http::StatusCode, Json,
    response::IntoResponse
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

pub async fn login(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
    Json(request): Json<LoginRequest>
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {

    // Validations
    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (cookie_jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (cookie_jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (cookie_jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (cookie_jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // handle request based on user's 2FA configuration
    match user.requires_2fa {
        true  => handle_2fa(cookie_jar).await,
        false => handle_no_2fa(&user.email, cookie_jar).await,
    }

}

async fn handle_2fa(jar: CookieJar) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    
    let auth_response = TwoFactorAuthResponse {
        message: String::from("2FA required"),
        login_attempt_id: String::from("123456"),
    };
    let response = Json(LoginResponse::TwoFactorAuth(auth_response));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
    
}

async fn handle_no_2fa(email: &Email, jar: CookieJar) -> (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return(jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))

}

