use std::sync::Arc;
use sqlx::PgPool;
use tokio::sync::RwLock;
use auth_service::{
    app_state::AppState,
    utils::constants::DATABASE_URL,
    get_postgres_pool,
    services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient},
    utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {

    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(MockEmailClient);
    let pg_pool = configure_postgresql().await;

    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
    // This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");

}

async fn configure_postgresql() -> PgPool {
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");
    pg_pool
}