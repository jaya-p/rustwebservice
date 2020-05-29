// build: cd <project-root-directory> && cargo build
// run: cd <project-root-directory> && cargo run --bin main
// run (other): cd <project-root-directory> && cargo run

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    rustwebservice::httpserver::httpserver(addr).await;
}

