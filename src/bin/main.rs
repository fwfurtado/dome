use dome::{endpoint, http, routes, Config, Environment};
use endpoint::secret;

#[tokio::main]
async fn main() {
    let router = routes! {
        "/secrets" => secret::routes()
    };

    http::start(&Environment::Local, Config::default(), router).await;
}
