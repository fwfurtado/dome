use dome::{endpoint, http, routes, Config};
use endpoint::secret;

#[tokio::main]
async fn main() {
    let router = routes! {
        "/secrets" => secret::routes()
    };

    http::start(Config::default(), router).await;
}
