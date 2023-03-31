use ::config::{Config, File};
use clap::Parser;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod cli;
mod config;
mod protocol;
mod router;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let setting = Config::builder()
        .add_source(File::from(cli.config_file()))
        .set_override_option("server_name", cli.server_name())
        .unwrap()
        .build()?;
    let config = config::ServerConfig::new(setting)?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(LevelFilter::DEBUG)
        .init();
    info!("Config is: {:?}", config);
    Ok(())
}
