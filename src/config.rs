use anyhow::anyhow;
use config::Config;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub server_name: String,
    pub bind_address: String,
}

impl ServerConfig {
    pub fn new(setting: Config) -> anyhow::Result<Self> {
        setting.try_deserialize().map_err(|err| anyhow!("{}", err))
    }
}

impl Display for ServerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "server_name: {} \n bind_address: {}",
            self.server_name, self.bind_address
        )
    }
}
