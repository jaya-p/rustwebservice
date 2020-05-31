// build: cd <project-root-directory> && cargo build
// run unit test: cd <project-root-directory> && cargo test

// helloworld module; just return "helloworld" String
pub mod helloworld;

// httpserver module prepares "helloworld" String as http service
//  and runs it with helloworld http service
pub mod httpserver;

// restapihttpserver module prepares "helloworld" String as REST API 
//  http service and runs it with helloworld http service
pub mod restapihttpserver;