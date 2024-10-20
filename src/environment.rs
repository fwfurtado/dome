pub enum Environment {
    Local,
    Staging,
    Production,
}

impl Environment {
    pub fn is_deployed(&self) -> bool {
        match self {
            Environment::Local => false,
            _ => true,
        }
    }
}
