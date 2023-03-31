use std::path::PathBuf;

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
    pub fn config_file(&self) -> PathBuf {
        self.config_file.clone()
    }

    pub fn server_name(&self) -> Option<String> {
        self.server_name.clone()
    }
}
