#![allow(unused)]

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("-->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();

    // handler hello
    // pass parameter
    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    async fn handler_hello(Query(param): Query<HelloParams>) -> impl IntoResponse {
        println!("-->> {:<12} - handler_hello - {param:?}", "HANDLER");

        let name = param.name.as_deref().unwrap_or("Bang!");
        Html(format!("Hello <strong>{name}</strong>"))
    }
}
