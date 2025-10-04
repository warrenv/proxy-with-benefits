use reqwest::Client;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use wiremock::MockServer;

use load_balancer::domain::LoadBalancer;
use load_balancer::{app_state::AppState, utils::constants::test, Application};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub clean_up_called: bool,
}

impl TestApp {
    pub async fn new() -> Self {
        let worker_hosts = vec![
            "http://localhost:7701".to_string(),
            "http://localhost:7702".to_string(),
        ];

        let load_balancer =
            LoadBalancer::new(worker_hosts).expect("failed to create load balancer");

        let app_state = AppState::new(load_balancer);

        let app = Application::build(
            app_state,
            //            load_balancer,
            &format!("{}:{}", test::LB_IP_ADDR, test::LB_PORT),
        )
        .await
        .expect("Failed to build app");
        //        let app = Application::build(app_state, test::LB_IP_ADDR, test::LB_PORT)
        //            .await
        //            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::builder().build().unwrap();

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
