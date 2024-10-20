use dome::{routes, Config, Environment};
use dome::{secret, serve_http};

#[tokio::main]
async fn main() {
    let router = routes! {
        "/secrets" => secret::routes()
    };

    serve_http(&Environment::Local, Config::default(), router).await;
}
