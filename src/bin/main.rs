use dome::config::Config;
use dome::endpoint::secret;
use dome::environment::Environment;
use dome::http::serve_http;
use dome::routes;

#[tokio::main]
async fn main() {
    let router = routes! {
        "/secrets" => secret::routes()
    };

    serve_http(&Environment::Local, Config::default(), router).await;
}
