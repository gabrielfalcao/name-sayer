#![allow(unused)]
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming as IncomingBody;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response};
use iocore::Path;
use tokio::net::TcpListener;
use white_box::io::TokioIo;
type Counter = i32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let svc = Svc {
        counter: Arc::new(Mutex::new(0)),
    };

    loop
    {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let svc_clone = svc.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, svc_clone)
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

#[derive(Debug, Clone)]
struct Svc
{
    counter: Arc<Mutex<Counter>>,
}

impl Service<Request<IncomingBody>> for Svc
{
    type Error = hyper::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;
    type Response = Response<Full<Bytes>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future
    {
        fn mk_response(s: String)
        -> Result<Response<Full<Bytes>>, hyper::Error>
        {
            Ok(Response::new(Full::new(Bytes::from(s))))
        }

        if req.uri().path() != "/favicon.ico"
        {
            *self.counter.lock().expect("lock poisoned") += 1;
        }

        let res = match req.uri().path()
        {
            "/" => mk_response(format!("home! counter = {:?}", self.counter)),
            "/posts" => mk_response(format!(
                "posts, of course! counter = {:?}",
                self.counter
            )),
            "/authors" => mk_response(format!(
                "authors extraordinare! counter = {:?}",
                self.counter
            )),
            _ => mk_response("oh no! not found".into()),
        };

        eprintln!("\x1b[1;48;2;167;223;216m\x1b[1;38;2;0;0;0m{req:#?}");
        // eprintln!("\x1b[1;48;2;155;193;93m\x1b[1;38;2;0;0;0m{res:#?}");
        eprintln!("\x1b[1;48;2;221;114;97m\x1b[1;38;2;0;0;0m{res:#?}");

        // let req_path =

        Box::pin(async { res })
    }
}
