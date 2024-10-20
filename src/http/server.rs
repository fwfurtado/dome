use crate::{Config, ServerConfig};
use axum::{serve, Router};
use tokio::net::TcpListener;

pub async fn start(
    config: Config<ServerConfig>,
    router: Router,
) {
    let addr = config.get().address();

    let listener = TcpListener::bind(addr.as_str()).await.unwrap();

    serve(listener, router).await.unwrap();
}
