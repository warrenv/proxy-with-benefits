//use axum::{
//    http::{Method, StatusCode},
//    response::{IntoResponse, Response},
//    routing::post,
//    serve::Serve,
//    Json, Router,
//};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{server::conn::http1, service::service_fn};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::RwLock; //, TcpStream};

use crate::{
    app_state::AppState,
    //domain::LoadBalancer,
    //routes::{login, logout, signup, verify_2fa, verify_token},
};

use crate::domain::LoadBalancer;

pub mod app_state;
pub mod domain;
pub mod utils;

//async fn handle(
//    req: Request<Body>,
//    load_balancer: Arc<RwLock<LoadBalancer>>,
//) -> Result<Response<Body>, hyper::Error> {
//    load_balancer.write().await.forward_request(req).await
//}

async fn handle(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

// This struct encapsulates our application-related logic.
pub struct Application {
    app_state: AppState,
    load_balancer: Arc<RwLock<LoadBalancer>>,
    //    server: Option<()>,
    address: String,
}

impl Application {
    //    pub async fn build_axum(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
    //        let allowed_origins = ["http://localhost:7777".parse()?];
    //
    //        let cors = CorsLayer::new()
    //            .allow_methods([Method::GET, Method::POST])
    //            .allow_credentials(true)
    //            .allow_origin(allowed_origins);
    //
    //        let router = Router::new()
    //            .nest_service("/", ServeDir::new("assets"))
    //            .with_state(app_state)
    //            .layer(cors)
    //            .layer(
    //                TraceLayer::new_for_http()
    //                    .make_span_with(make_span_with_request_id)
    //                    .on_request(on_request)
    //                    .on_response(on_response),
    //            );
    //
    //        let listener = tokio::net::TcpListener::bind(address).await?;
    //        let address = listener.local_addr()?.to_string();
    //        let server = axum::serve(listener, router);
    //
    //        Ok(Application { server, address })
    //    }

    pub async fn build(
        app_state: AppState,
        load_balancer: Arc<RwLock<LoadBalancer>>,
        _address: &str,
    ) -> Result<Self, Box<dyn Error>> {
        //let worker_hosts = vec![
        //    "http://localhost:7701".to_string(),
        //    "http://localhost:7702".to_string(),
        //];

        //let load_balancer = Arc::new(RwLock::new(
        //    LoadBalancer::new(worker_hosts).expect("failed to create load balancer"),
        //));

        // TODO: get these from app_state.config
        let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 1337));

        //        let server = None;
        //let server =
        //    Server::bind(&addr).serve(make_service_fn(move |_conn| {
        //        let load_balancer = app_state.load_balancer.clone();
        //        async move {
        //            Ok::<_, Infallible>(service_fn(move |req| handle(req, load_balancer.clone())))
        //        }
        //    }));

        Ok(Application {
            app_state,
            load_balancer,
            //            server,
            address: address.to_string(),
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        // Bind to the port and listen for incoming TCP connections
        let listener = TcpListener::bind(self.address).await?;

        loop {
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
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(handle))
                    //.serve_connection(io, service_fn(self.app_state.load_balancer.forward_request))
                    .await
                {
                    eprintln!("Error serving connection: {}", err);
                }
            });
        }
    }

    //    pub async fn run0(self) -> Result<(), std::io::Error> {
    //        tracing::info!("listening on {}", &self.address);
    //        match self.server {
    //            Some(_) => {
    //                tracing::info!("server will run on {}.", self.address);
    //                Ok(())
    //            }
    //            None => {
    //                tracing::info!("No server was built on {}.", self.address);
    //                Ok(())
    //            }
    //        }
    //    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

//impl IntoResponse for LoadBalancerError {
//    fn into_response(self) -> Response {
//        log_error_chain(&self);
//
//        let (status, error_message) = match self {
//            LoadBalancerError::UnexpectedError(_) => {
//                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
//            }
//        };
//
//        let body = Json(ErrorResponse {
//            error: error_message.to_string(),
//        });
//
//        (status, body).into_response()
//    }
//}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}
