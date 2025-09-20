use std::{convert::Infallible, net::SocketAddr, str::FromStr, sync::Arc};

use hyper::{
    client::ResponseFuture,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server, Uri,
};
use tokio::sync::RwLock;

use proxy_with_benefits::{
    app_state::Appstate, domain::LoadBalancer, utils::contants::prod, Application,
};

async fn handle(
    req: Request<Body>,
    load_balancer: Arc<RwLock<LoadBalancer>>,
) -> Result<Response<Body>, hyper::Error> {
    load_balancer.write().await.forward_request(req).await
}

#[tokio::main]
async fn main() {
    let worker_hosts = vec![
        "http://localhost:3000".to_string(),
        "http://localhost:3001".to_string(),
    ];

    let load_balancer = Arc::new(RwLock::new(
        LoadBalancer::new(worker_hosts).expect("failed to create load balancer"),
    ));

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 1337));

    let server = Server::bind(&addr).serve(make_service_fn(move |_conn| {
        let load_balancer = load_balancer.clone();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle(req, load_balancer.clone()))) }
    }));

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}
