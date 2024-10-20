use crate::logging::Level;

#[derive(Clone, Debug)]
pub struct ServerConfig {
    host: String,
    port: u16,
    log_level: Level,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new("0.0.0.0".to_string(), 3000, Level::Debug)
    }
}

impl ServerConfig {
    pub fn new(
        host: String,
        port: u16,
        log_level: Level,
    ) -> Self {
        Self {
            host,
            port,
            log_level,
        }
    }

    pub fn log_level(&self) -> Level {
        self.log_level.clone()
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
