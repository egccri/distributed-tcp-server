use anyhow::anyhow;
use config::Config;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub server_name: String,
    pub bind_address: String,
    pub router: RouterConfig,
    pub raft: Option<RaftConfig>,
    pub redis: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RouterConfig {
    pub router_id: u64,
    pub router_server_addr: String,
    pub keep_alive_timeout: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RaftConfig {
    pub node_id: u64,
    pub raft_network_addr: String,
    pub heartbeat_interval: u32,
    pub election_timeout_min: u32,
    pub election_timeout_max: u32,
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
            "server_name: {} \n bind_address: {} \n \
            router_config: {:?} \n raft_config: {:?} \n redis: {:?}",
            self.server_name, self.bind_address, self.router, self.raft, self.redis
        )
    }
}
