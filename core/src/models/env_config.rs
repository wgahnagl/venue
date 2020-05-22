use std::env;

pub struct ServerConfig {
    pub database_url: String,
    pub bind_address: String,
}

impl ServerConfig {
    pub fn from_env() -> Option<Self> {
        let database_url = env::var("MYSQL_DATABASE_URL").ok()?;
        let bind_address = env::var("BIND_ADDRESS").ok()?;
        Some(ServerConfig {
            database_url,
            bind_address,
        })
    }
}