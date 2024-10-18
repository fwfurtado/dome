use crate::{Config, ServerConfig};
use axum::{serve, Router};
use tokio::net::TcpListener;

pub fn router(nested: Vec<(&str, fn() -> Router)>) -> Router {
    let mut router = Router::new();

    for (path, route_fn) in nested {
        router = router.nest(path, route_fn());
    }

    router
}

pub async fn start(
    config: Config<ServerConfig>,
    router: Router,
) {
    let addr = config.get().address();

    let listener = TcpListener::bind(addr.as_str()).await.unwrap();

    serve(listener, router).await.unwrap();
}
