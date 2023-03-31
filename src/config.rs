use anyhow::anyhow;
use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub server_name: String,
}

impl ServerConfig {
    pub fn new(setting: Config) -> anyhow::Result<Self> {
        setting.try_deserialize().map_err(|err| anyhow!("{}", err))
    }
}
