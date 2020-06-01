// build: cd <project-root-directory> && cargo build
// run: cd <project-root-directory> && cargo run --bin main
// run (other): cd <project-root-directory> && cargo run

use std::net::SocketAddr;

// #[tokio::main] macro = main() function will be run on tokio runtime
//   main() run as async (runs as non-blocking task)
// note: async code allows us to run multiple tasks concurrently
//   on the same OS thread.
#[tokio::main]
async fn main() {
    // define socket (IP + Port) address for http server to listen to
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // run http server on above socket address
    rustwebservice::restapihttpserver::httpserver(addr).await;
}

