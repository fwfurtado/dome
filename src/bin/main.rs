use dome::{endpoint, http, Config};
use endpoint::secret;

#[tokio::main]
async fn main() {
    let router = http::router(vec![("/secrets", secret::routes)]);

    http::start(Config::default(), router).await;
}
