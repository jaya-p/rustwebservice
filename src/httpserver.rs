use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use super::helloworld;

async fn helloworldwebservice(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  Ok(Response::new(Body::from(helloworld::helloworld())))
}

pub async fn httpserver(addr: SocketAddr) {
  let server_future = Server::bind(&addr)
    .serve(make_service_fn(|_| async {
      Ok::<_, hyper::Error>(service_fn(helloworldwebservice))
    }));
  
  println!("helloworld webserver is running");
  let r = server_future.await;
  if r.is_err() {
    eprintln!("helloworld webserver error: {}", r.err().unwrap());
  }
}
