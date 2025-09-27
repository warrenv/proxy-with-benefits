use hyper::{service::service_fn, Request, Response};
use std::{convert::Infallible, net::SocketAddr, str::FromStr, sync::Arc};
use tokio::sync::RwLock;

use load_balancer::{
    app_state::AppState,
    domain::LoadBalancer,
    utils::constants::{DEFAULT_LB_IP_ADDR, DEFAULT_LB_PORT},
    Application,
};

//mod app_state;

async fn handle(
    req: Request<hyper::body::Incoming>,
    load_balancer: Arc<RwLock<LoadBalancer>>,
    //) -> Result<Response<hyper::body::Incoming>, hyper::Error> {
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = load_balancer.write().await;
    LoadBalancer::forward_request(req).await
}

#[tokio::main]
async fn main() {
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
