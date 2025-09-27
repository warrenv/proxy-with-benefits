//use hyper::{client::ResponseFuture, Body, Client, Request, Uri};
use http_body_util::Full;
use hyper::{body::Bytes, Request, Response, Uri};
use tokio::io::{self, AsyncWriteExt as _};
//use reqwest::Client;
use http_body_util::{BodyExt, Empty};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::str::FromStr;
use tokio::net::TcpStream;

#[derive(Clone)]
pub struct LoadBalancer {
    //client: Client<hyper::client::HttpConnector>,
    //    client: http1,
    //client: Client,
    worker_hosts: Vec<String>,
    current_worker: usize,
}

impl LoadBalancer {
    pub fn new(worker_hosts: Vec<String>) -> Result<Self, String> {
        if worker_hosts.is_empty() {
            return Err("No worker hosts provided".into());
        }

        Ok(LoadBalancer {
            //            client: Client::new(),
            worker_hosts,
            current_worker: 0,
        })
    }

    //async fn fetch_url(url: hyper::Uri) -> Result<()> {
    pub async fn forward_request(
        req: Request<hyper::body::Incoming>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let host = "example.com"; //req.host().expect("uri has no host");
        let port = 80; //req.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        //let authority = url.authority().unwrap().clone();

        let path = "/"; //url.path();
        let req = Request::builder()
            .uri(path)
            //.header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;

        let mut res = sender.send_request(req).await?;

        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        // Stream the body, writing each chunk to stdout as we get it
        // (instead of buffering and printing at the end).
        while let Some(next) = res.frame().await {
            let frame = next?;
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk).await?;
            }
        }

        println!("\n\nDone!");

        Ok(())
    }
    //    pub async fn forward_request(
    //        &mut self,
    //        req: Request<hyper::body::Incoming>,
    //    ) -> Result<Response<hyper::body::Incoming>, hyper::Error> {
    //        let mut worker_uri = self.get_worker().to_owned();
    //
    //        // Extract the path and query from the original request
    //        if let Some(path_and_query) = req.uri().path_and_query() {
    //            worker_uri.push_str(path_and_query.as_str());
    //        }
    //
    //        // Create a new URI from the worker URI
    //        let new_uri = Uri::from_str(&worker_uri).unwrap();
    //
    //        // Extract the headers from the original request
    //        let headers = req.headers().clone();
    //
    //        // Clone the original request's headers and method
    //        let mut new_req = Request::builder()
    //            .method(req.method())
    //            .uri(new_uri)
    //            .body(req.into_body())
    //            .expect("request builder");
    //
    //        // Copy headers from the original request
    //        for (key, value) in headers.iter() {
    //            new_req.headers_mut().insert(key, value.clone());
    //        }
    //
    //        let addr = format!("{}:{}", "127.0.0.1".to_string(), "7701".to_string());
    //
    //        async move {
    //            let client_stream = TcpStream::connect(addr).await.unwrap();
    //            let io = TokioIo::new(client_stream);
    //
    //            let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    //            tokio::task::spawn(async move {
    //                if let Err(err) = conn.await {
    //                    println!("Connection failed: {:?}", err);
    //                }
    //            });
    //
    //            sender.send_request(req).await
    //        }
    //
    //        //self.client.request(new_req)
    //        //self.client.get(new_uri).send().await
    //    }

    fn get_worker(&mut self) -> &str {
        // Use a round-robin strategy to select a worker
        let worker = self.worker_hosts.get(self.current_worker).unwrap();
        self.current_worker = (self.current_worker + 1) % self.worker_hosts.len();
        worker
    }
}
