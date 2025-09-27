use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, trace, warn};

use load_balancer::{
    app_state::AppState,
    domain::LoadBalancer,
    utils::constants::{DEFAULT_LB_IP_ADDR, DEFAULT_LB_PORT},
    Application,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting app");

    let worker_hosts = vec![
        "http://localhost:7701".to_string(),
        "http://localhost:7702".to_string(),
    ];

    let load_balancer = Arc::new(RwLock::new(
        LoadBalancer::new(worker_hosts).expect("failed to create load balancer"),
    ));

    let app_state = AppState::new();

    let app = Application::build(
        app_state,
        load_balancer,
        &format!("{}:{}", DEFAULT_LB_IP_ADDR, DEFAULT_LB_PORT),
    )
    .await
    .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
