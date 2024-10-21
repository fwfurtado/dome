use crate::environment::Environment;

#[derive(Clone, Debug)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Into<tracing::Level> for Level {
    fn into(self) -> tracing::Level {
        match self {
            Level::Debug => tracing::Level::DEBUG,
            Level::Info => tracing::Level::INFO,
            Level::Warn => tracing::Level::WARN,
            Level::Error => tracing::Level::ERROR,
        }
    }
}

pub fn register_tracing(
    env: &Environment,
    log_level: &Level,
) {
    let log_level: tracing::Level = log_level.clone().into();

    if env.is_deployed() {
        tracing_subscriber::fmt()
            .with_max_level(log_level)
            .json()
            .init();
    } else {
        tracing_subscriber::fmt().with_max_level(log_level).init();
    }
}
