use crate::config::ServerConfig;
use std::path::PathBuf;
use tokio::sync::{broadcast, mpsc};

/// Cli commands
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Server config file path, the default is "./config.toml"
    #[arg(
        short = 'c',
        long = "config-file",
        default_value = "config/config.toml"
    )]
    config_file: PathBuf,

    /// Server name
    #[arg(short = 'n', long = "server-name")]
    server_name: Option<String>,
}

impl Cli {
    pub async fn execute(self, server_config: ServerConfig) -> anyhow::Result<()> {
        // terminal -> server
        let (ctrl_c_tx, ctrl_c_rx) = broadcast::channel(5);
        // server -> router
        let (server_sender, server_receiver) = mpsc::channel(1000);

        let server_config_clone = server_config.clone();
        crate::server::start(server_config_clone, server_sender, ctrl_c_rx).await?;
        Ok(())
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_file.clone()
    }

    pub fn server_name(&self) -> Option<String> {
        self.server_name.clone()
    }
}
