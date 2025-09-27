//use crate::domain::LoadBalancer;
//use std::sync::Arc;
//use tokio::sync::RwLock;

//pub type LoadBalancerType = Arc<RwLock<dyn LoadBalancer + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    //pub load_balancer: LoadBalancerType,
}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
    //    pub fn new(load_balancer: LoadBalancerType) -> Self {
    //        Self { load_balancer }
    //    }
}
