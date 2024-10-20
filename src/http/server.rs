use crate::config::Config;
use crate::environment::Environment;
use crate::http::ServerConfig;
use crate::logging;
use axum::{serve, Router};
use log::info;
use tokio::net::TcpListener;

pub async fn serve_http(
    env: &Environment,
    config: Config<ServerConfig>,
    router: Router,
) {
    let cfg = config.get();
    let addr = cfg.address();

    logging::register_tracing(&env, &cfg.log_level());

    let listener = TcpListener::bind(addr.as_str()).await.unwrap();

    info!("Listening on: {}", addr);

    serve(listener, router).await.unwrap();
}
