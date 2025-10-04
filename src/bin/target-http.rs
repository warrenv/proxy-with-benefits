#![deny(warnings)]

//use bytes::Bytes;
use http_body_util::Empty;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
//use hyper::{body::Incoming as IncomingBody, header, Method, Request, Response, StatusCode};
use hyper::{Method, Request, Response, StatusCode};
//use hyper::{Request, Response};
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper_util::rt::TokioIo;
use hyper_util::rt::TokioTimer;
//use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::time::{sleep, Duration};

//async fn echo(
async fn response_examples(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        //        (&Method::GET, "/") => Ok(Response::new(full("Try POSTing data to /echo"))),
        (&Method::GET, "/health") => {
            println!("got a health check request");
            let mut found = Response::new(empty());
            *found.status_mut() = StatusCode::OK;
            Ok(found)
        }

        (&Method::GET, "/") => hello(req).await,
        //        (&Method::POST, "/") => {
        //            // we'll be back
        //            let mut found = Response::new(empty());
        //            *found.status_mut() = StatusCode::OK;
        //            Ok(found)
        //        }

        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

//async fn response_examples(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
//async fn response_examples2(
//    req: Request<hyper::body::Incoming>,
//    //) -> Result<Response<Full<Bytes>>, Infallible> {
//) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
//    println!("req: {:?}", req);
//
//    match (req.method(), req.uri().path()) {
//        (&Method::GET, "/") => hello(req).await,
//        (&Method::GET, "/health") => health(req).await,
//        _ => {
//            // Return 404 not found response.
//            Ok(Response::builder()
//                .status(StatusCode::NOT_FOUND)
//                .body(Full::new(Bytes::from("Not found")))
//                .boxed())
//        }
//    }
//}

// An async function that consumes a request, does nothing with it and returns a
// response.
//async fn hello(req: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
//async fn hello(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
async fn hello(
    req: Request<hyper::body::Incoming>,
    //) -> Result<BoxBody<Bytes, hyper::Error>, Infallible> {
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let delay: u64 = env::var("DELAY").unwrap().parse().unwrap();
    sleep(Duration::from_millis(delay)).await;
    //Ok(Response::new(Full::new(Bytes::from(
    //    "Hello from http delay!\n",
    //))))

    println!("REQ REQ: {:?}", req);
    //let (parts, body) = req.into_parts();
    //let body_bytes = hyper::body::to_bytes(body).await.unwrap();
    //let body_bytes = req.into_body();
    //    println!("REQ BODY: {:?}", body_bytes);
    //println!("REQ BODY: {:?}", req.body().collect().await?.to_bytes(););
    //println!("REQ BODY: {:?}", Bytes::from(req.body()));
    //    let whole_body = req.collect().await?.to_bytes();
    //    println!("BODY: {}", whole_body);

    Ok(Response::new(req.into_body().boxed()))

    //    Ok(Response::new(Full::new(Bytes::from(format!(
    //        "Hello from http delay!\n",
    //    )))))
}

//async fn health(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
//async fn health(
//    _: Request<impl hyper::body::Body>,
//) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
//    Ok(Response::builder()
//        .status(StatusCode::OK)
//        .body(Full::new(Bytes::from(""))))
//}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // This address is localhost
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();

    // Bind to the port and listen for incoming TCP connections
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        // When an incoming TCP connection is received grab a TCP stream for
        // client<->server communication.
        //
        // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
        // .await point allows the Tokio runtime to pull the task off of the thread until the task
        // has work to do. In this case, a connection arrives on the port we are listening on and
        // the task is woken up, at which point the task is then put back on a thread, and is
        // driven forward by the runtime, eventually yielding a TCP stream.
        let (tcp, _) = listener.accept().await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(tcp);

        // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
        // current task without waiting for the processing of the HTTP1 connection we just received
        // to finish
        tokio::task::spawn(async move {
            // Handle the connection from the client using HTTP1 and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http1::Builder::new()
                .timer(TokioTimer::new())
                //.serve_connection(io, service_fn(hello))
                .serve_connection(io, service_fn(response_examples))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

// fit our broadened Response body type.
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn _full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
