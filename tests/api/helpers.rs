use reqwest::Client;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use wiremock::MockServer;

use auth_service::{app_state::AppState, utils::constants::test, Application};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub clean_up_called: bool,
}

impl TestApp {
    pub async fn new() -> Self {
        let app_state = AppState {};

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        Self {
            address,
            http_client,
            clean_up_called: false,
        }
    }

    pub async fn clean_up(&mut self) {
        self.clean_up_called = true
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
