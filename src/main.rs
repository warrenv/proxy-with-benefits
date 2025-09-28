use load_balancer::{
    app_state::AppState,
    domain::LoadBalancer,
    utils::constants::{DEFAULT_LB_IP_ADDR, DEFAULT_LB_PORT},
    Application,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Creating LoadBalancer");

    let worker_hosts = vec![
        "http://localhost:7701".to_string(),
        "http://localhost:7702".to_string(),
    ];

    let load_balancer = LoadBalancer::new(worker_hosts).expect("failed to create load balancer");

    let app_state = AppState::new(load_balancer);

    tracing::info!("Creating Application");

    let app = Application::build(
        app_state,
        &format!("{}:{}", DEFAULT_LB_IP_ADDR, DEFAULT_LB_PORT),
    )
    .await
    .expect("Failed to build app");

    tracing::info!("Running load balancer");

    app.run().await.expect("Failed to run app");
}
