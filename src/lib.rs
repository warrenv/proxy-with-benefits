use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{server::conn::http1, service::service_fn};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::RwLock; //, TcpStream};

use crate::app_state::AppState;
use crate::domain::LoadBalancer;

pub mod app_state;
pub mod domain;
pub mod utils;

async fn handle(
    load_balancer: Arc<RwLock<LoadBalancer>>,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(load_balancer
        .write()
        .await
        .forward_request(req)
        .await
        .unwrap())
}

// This struct encapsulates our application-related logic.
pub struct Application {
    app_state: AppState,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, _address: &str) -> Result<Self, Box<dyn Error>> {
        // TODO: get these from app_state.config
        let port: u16 = env::var("PORT").unwrap().parse().unwrap();
        let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

        Ok(Application {
            app_state,
            address: address.to_string(),
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);

        // Bind to the port and listen for incoming TCP connections
        let listener = TcpListener::bind(self.address).await?;

        loop {
            let load_balancer = Arc::clone(&self.app_state.load_balancer);

            // When an incoming TCP connection is received grab a TCP stream for
            // client-server communication.
            //
            // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
            // .await point allows the Tokio runtime to pull the task off of the thread until the task
            // has work to do. In this case, a connection arrives on the port we are listening on and
            // the task is woken up, at which point the task is then put back on a thread, and is
            // driven forward by the runtime, eventually yielding a TCP stream.
            let (stream, _) = listener.accept().await?;

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
            // current task without waiting for the processing of the HTTP/2 connection we just received
            // to finish

            tokio::task::spawn(async move {
                // Handle the connection from the client using HTTP/2 with an executor and pass any
                // HTTP requests received on that connection to the `hello` function
                //if let Err(err) = http2::Builder::new(TokioExecutor)

                let service = service_fn(|req| handle(load_balancer.clone(), req));

                if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                    tracing::error!("Error serving connection: {}", err);
                }
            });
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
