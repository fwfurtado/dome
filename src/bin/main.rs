use dome::config::Config;
use dome::domain::Cipher;
use dome::endpoint::secret;
use dome::environment::Environment;
use dome::gateway::secret::SecretGateway;
use dome::http::serve_http;
use dome::routes;
use dome::usecase::secret::SecretUseCase;

#[tokio::main]
async fn main() {
    let use_case = SecretUseCase::new(Cipher::default(), SecretGateway::new());

    let router = routes! {
        "/secrets" => secret::routes(&use_case)
    };

    serve_http(&Environment::Local, Config::default(), router).await;
}
