// test: cargo test test_httpserver

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use super::helloworld;

// helloworldwebservice wraps helloworld() as hyper http service
//  helloworldwebservice is async (runs as non-blocking task) and private
async fn helloworldwebservice(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  Ok(Response::new(Body::from(helloworld::helloworld())))
}

// httpserver provides http server function
//   httpserver is async (runs as non-blocking task) and public
pub async fn httpserver(addr: SocketAddr) {
  // server_future specifies server configuration 
  //   and service to be run (in this case, helloworldwebservice())
  let server_future = Server::bind(&addr)
    .serve(make_service_fn(|_| async {
      Ok::<_, hyper::Error>(service_fn(helloworldwebservice))
    }));
  
  // server_future runs here, with error handling defined
  println!("helloworld webserver is running");
  let r = server_future.await;
  if r.is_err() {
    eprintln!("helloworld webserver error: {}", r.err().unwrap());
  }
}

// #[cfg(test)] is attribute marker for test module
#[cfg(test)]
mod tests {
  // importing names from outer (for mod tests) scope.
  use super::*;
  use tokio::runtime::Runtime;

  // #[test] is attribute marker for test function
  #[test]
  #[ignore = "only 1 tokio runtime can be run. and it is used by restapi test"]
  fn test_httpserver() -> Result<(), reqwest::Error> {
    
    // use tokio runtime for running httpserver
    // https://docs.rs/tokio/0.2.21/tokio/runtime/struct.Runtime.html
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
      let sockaddr = SocketAddr::from(([127, 0, 0, 1], 8080));
      httpserver(sockaddr).await;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));

    let resp = reqwest::blocking::get("http://localhost:8080/")?.text()?;
    assert_eq!(resp, "Hello World");

    Ok(())
  }
}
