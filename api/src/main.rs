use axum::{response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;
use tokio::net::TcpListener;

pub use self::error::Result;

mod error;
mod web;

async fn shutdown_signal() {
  tokio::signal::ctrl_c().await.unwrap();
}

async fn ping_handler() -> impl IntoResponse {
  (StatusCode::OK, "PONG")
}

#[tokio::main]
async fn main() -> Result<()> {
  let routers = Router::new().route("/ping", get(ping_handler));

  let tcp_listener = TcpListener::bind("localhost:8000").await.unwrap();
  axum::serve(tcp_listener, routers.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

  Ok(())
}
