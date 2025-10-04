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

//async fn handle(req: Request<hyper::body::Incoming>) -> Result<Response<Body>, hyper::Error> {
//async fn handle(
//    context: AppContext,
//    addr: SocketAddr,
//    req: Request<Body>
//) -> Result<Response<Body>, Infallible> {
//    Ok(Response::new(Body::from("Hello World")))
//}

// use this when we add more routes to the proxy.
//async fn response_examples(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
//    match (req.method(), req.uri().path()) {
//        (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(full(INDEX))),
//        (&Method::GET, "/test.html") => client_request_response().await,
//        (&Method::POST, "/json_api") => api_post_response(req).await,
//        (&Method::GET, "/json_api") => api_get_response().await,
//        _ => {
//            // Return 404 not found response.
//            Ok(Response::builder()
//                .status(StatusCode::NOT_FOUND)
//                .body(full(NOTFOUND))
//                .unwrap())
//        }
//    }
//}

async fn handle(
    load_balancer: Arc<RwLock<LoadBalancer>>,
    mut req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(load_balancer
        .write()
        .await
        .forward_request(req)
        .await
        .unwrap())
}

//async fn handle(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
//    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
//}

// This struct encapsulates our application-related logic.
pub struct Application {
    app_state: AppState,
    //    load_balancer: Arc<RwLock<LoadBalancer>>,
    pub address: String,
}

impl Application {
    pub async fn build(
        app_state: AppState,
        //        load_balancer: Arc<RwLock<LoadBalancer>>,
        _address: &str,
    ) -> Result<Self, Box<dyn Error>> {
        // TODO: get these from app_state.config
        let port: u16 = env::var("PORT").unwrap().parse().unwrap();
        let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

        Ok(Application {
            app_state,
            //           load_balancer,
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

            //let foo = &self.load_balancer.worker_hosts; //vec!["http://example.com".to_string()];

            //let foo: Vec<String> = self.load_balancer.worker_hosts.into(); //vec!["http://example.com".to_string()];

            //let foo: Vec<String> = vec!["a".into(), "bc".into(), "de".into()];

            //let foo = &self.load_balancer.worker_hosts;
            //let foo = &self.load_balancer;

            tokio::task::spawn(async move {
                // Handle the connection from the client using HTTP/2 with an executor and pass any
                // HTTP requests received on that connection to the `hello` function
                //if let Err(err) = http2::Builder::new(TokioExecutor)

                let service = service_fn(|mut req| handle(load_balancer.clone(), req));

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

//fn log_error_chain(e: &(dyn Error + 'static)) {
//    let separator =
//        "\n-----------------------------------------------------------------------------------\n";
//    let mut report = format!("{}{:?}\n", separator, e);
//    let mut current = e.source();
//    while let Some(cause) = current {
//        let str = format!("Caused by:\n\n{:?}", cause);
//        report = format!("{}\n{}", report, str);
//        current = cause.source();
//    }
//    report = format!("{}\n{}", report, separator);
//    tracing::error!("{}", report);
//}
