use std::sync::Arc;
use reqwest::cookie::Jar;
use tokio::sync::RwLock;
use uuid::Uuid;
use auth_service::{
    app_state::{AppState, UserStoreType},
    services::{
        HashmapUserStore,
        HashsetBannedTokenStore
    },
    Application,
};
use auth_service::utils::constants::test;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>, // Atomic reference counter
    pub banned_token_store: Arc<RwLock<HashsetBannedTokenStore>>,
    pub http_client: reqwest::Client,
}

impl TestApp {

    pub async fn new() -> Self {

        let hashmap_user_store = HashmapUserStore::new();
        let user_store: UserStoreType = Arc::new(RwLock::new(hashmap_user_store));
        let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let app_state = AppState::new(user_store, banned_token_store.clone());

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        // Create new `TestApp` instance and return it
        Self {
            address,
            cookie_jar,
            banned_token_store,
            http_client,
        }
    
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
        where Body: serde::Serialize {
            self.http_client
                .post(&format!("{}/signup", &self.address))
                .json(body)
                .send()
                .await
                .expect("Failed to execute POST signup.")
        }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
        where Body: serde::Serialize {
            self.http_client
                .post(&format!("{}/login", &self.address))
                .json(body)
                .send()
                .await
                .expect("Failed to execute POST login.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where Body: serde::Serialize {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}