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

#[derive(Clone, Debug)]
pub struct LoadBalancer {
    pub worker_hosts: Vec<String>,
    current_worker: usize,
}

impl LoadBalancer {
    pub fn new(worker_hosts: Vec<String>) -> Result<Self, String> {
        if worker_hosts.is_empty() {
            return Err("No worker hosts provided".into());
        }

        Ok(Self {
            worker_hosts,
            current_worker: 0,
        })
    }

    pub async fn forward_request(
        &mut self,
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
        let worker = self.get_worker();

        let (parts, _) = req.into_parts();
        //let (mut parts, body) = req.into_parts();
        //tracing::info!("parts: {:?}", parts);
        //tracing::info!("body: {:?}", body);

        //let host = "example.com"; //req.host().expect("uri has no host");
        //let port = 80; //req.port_u16().unwrap_or(80);
        //let addr = format!("{}:{}", host, port);
        //let stream = TcpStream::connect(addr).await?;
        let stream = TcpStream::connect(worker).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        tracing::info!("connection to {} established", worker);

        //let authority = url.authority().unwrap().clone();

        // 2025-09-27T21:00:12.928202Z  INFO load_balancer::domain::load_balancer: parts: Parts { method: GET, uri: /, version: HTTP/1.1, headers: {"host": "127.0.0.1:1337", "user-agent": "curl/8.14.1", "accept": "*/*"} }

        //let path = parts.uri; //"/"; //url.path();
        //        let req = Request::builder()
        //            .uri(path)
        //            //.header(hyper::header::HOST, authority.as_str())
        //            .body(Empty::<Bytes>::new())?;

        let request = Request::builder()
            .method(parts.method)
            .uri("https://example.com/")
            .body(Empty::<Bytes>::new())
            .unwrap();

        let res = sender.send_request(request).await?;
        tracing::info!("Response: {}", res.status());
        let body = res.collect().await?.to_bytes();

        Ok(Response::new(Full::new(Bytes::from(body))))
    }

    fn get_worker(&mut self) -> &str {
        // Use a round-robin strategy to select a worker
        let worker = self.worker_hosts.get(self.current_worker).unwrap();
        self.current_worker = (self.current_worker + 1) % self.worker_hosts.len();

        tracing::info!("{:?}", self);
        worker
    }
}
