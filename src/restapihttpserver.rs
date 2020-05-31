// test: cargo test test_gethelloworldwebservice 
// test: cargo test test_posthelloworldwebservice 
// test: cargo test test_statusnotfoundwebservice

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use std::net::SocketAddr;
use super::helloworld;

// gethelloworldwebservice wraps helloworld() return as http response
//  gethelloworldwebservice is async (runs as non-blocking task) and private
async fn gethelloworldwebservice(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  Ok(Response::new(Body::from(helloworld::helloworld())))
}

// posthelloworldwebservice wraps helloworld() return as http response
//  posthelloworldwebservice is async (runs as non-blocking task) and private
async fn posthelloworldwebservice(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  
  // _req is returning body object; 
  // because Body is stream of bytes; then we need wait until all bytes come
  //     and converted to bytes (hence await and to_bytes())
  // then we convert body bytes to string
  let whole_body_in_bytes = hyper::body::to_bytes(_req.into_body()).await?;
  let body_string = std::str::from_utf8(&whole_body_in_bytes).unwrap();

  // then we decode body string as JSON
  let json_data: serde_json::Value = serde_json::from_str(body_string).unwrap();
    
  // then if there is "Name" field, then we say hello with the name
  let input_name = if json_data["Name"].is_string() {
      json_data["Name"].as_str().unwrap().replace("\"","")
    } else {
      String::from("Anonymous")
    };
  // send the response :)
  Ok(Response::new(Body::from(format!("{}, {}", helloworld::helloworld(), input_name))))
}

// statusnotfoundwebservice wraps "404 not found" page as http response
//  statusnotfoundwebservice is async (runs as non-blocking task) and private
async fn statusnotfoundwebservice(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  Ok(Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(Body::from(String::from("404 Not Found")))
    .unwrap())
}

// webservicerouter
async fn webservicerouter(_req: Request<Body>) 
    -> Result<Response<Body>, hyper::Error> {
  match (_req.method(), _req.uri().path()) {
    (&Method::GET, "/api/v1/helloworld") => gethelloworldwebservice(_req).await,
    (&Method::POST, "/api/v1/helloworld") => posthelloworldwebservice(_req).await,
    _ => statusnotfoundwebservice(_req).await
  }  
}

// httpserver provides http server function
//   httpserver is async (runs as non-blocking task) and public
pub async fn httpserver(addr: SocketAddr) {
  // server_future specifies server configuration 
  //   and service to be run (in this case, helloworldwebservice())
  let server_future = Server::bind(&addr)
    .serve(make_service_fn(|_| async {
      Ok::<_, hyper::Error>(service_fn(webservicerouter))
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
  fn test_gethelloworldwebservice() -> Result<(), reqwest::Error> {
    
    // use tokio runtime for running httpserver
    // https://docs.rs/tokio/0.2.21/tokio/runtime/struct.Runtime.html
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
      let sockaddr = SocketAddr::from(([127, 0, 0, 1], 8080));
      httpserver(sockaddr).await;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));

    let resp = reqwest::blocking::get("http://localhost:8080/api/v1/helloworld")?.text()?;
    assert_eq!(resp, "Hello World");

    Ok(())
  }

  // #[test] is attribute marker for test function
  #[test]
  fn test_posthelloworldwebservice() -> Result<(), reqwest::Error> {
    
    // use tokio runtime for running httpserver
    // https://docs.rs/tokio/0.2.21/tokio/runtime/struct.Runtime.html
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
      let sockaddr = SocketAddr::from(([127, 0, 0, 1], 8080));
      httpserver(sockaddr).await;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));

    let client = reqwest::blocking::Client::new();
    let inputstr = "{\"Name\": \"There\"}"; 
    println!("inputstr: {}",inputstr);
    let resp = client.post("http://localhost:8080/api/v1/helloworld")
      .body(inputstr)
      .send()?;
    assert_eq!(resp.text()?, "Hello World, There");

    Ok(())
  }

  // #[test] is attribute marker for test function
  #[test]
  fn test_statusnotfoundwebservice() -> Result<(), reqwest::Error> {
    
    // use tokio runtime for running httpserver
    // https://docs.rs/tokio/0.2.21/tokio/runtime/struct.Runtime.html
    let rt = Runtime::new().unwrap();
    rt.spawn(async {
      let sockaddr = SocketAddr::from(([127, 0, 0, 1], 8080));
      httpserver(sockaddr).await;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));

    let client = reqwest::blocking::Client::new();
    let resp = client.delete("http://localhost:8080/api/v1/helloworld").send()?;
    assert_eq!(resp.text()?, "404 Not Found");

    Ok(())
  }
}
