use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::LoadBalancer;

//pub type LoadBalancerType = Arc<RwLock<dyn LoadBalancer + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    //pub load_balancer: LoadBalancerType,
    //pub load_balancer: LoadBalancer,
    pub load_balancer: Arc<RwLock<LoadBalancer>>,
}

impl AppState {
    pub fn new(load_balancer: LoadBalancer) -> Self {
        Self {
            load_balancer: Arc::new(RwLock::new(load_balancer)),
        }
    }
    //    pub fn new(load_balancer: LoadBalancerType) -> Self {
    //        Self { load_balancer }
    //    }
}
