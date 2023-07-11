use ::config::{Config, File};
use clap::Parser;
use time::UtcOffset;
use tracing::{info, Level};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod cli;
mod config;
mod panic_hook;
pub(crate) mod protocol;
mod router;
pub mod server;
mod storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    panic_hook::set_panic_hook();
    let cli = cli::Cli::parse();
    let setting = Config::builder()
        .add_source(File::from(cli.config_file()))
        .set_override_option("server_name", cli.server_name())
        .unwrap()
        .build()?;
    let config = config::ServerConfig::new(setting)?;
    let daily_appender =
        tracing_appender::rolling::daily("./logs", config.server_name.clone() + ".log");
    let (daily_no_blocking, _guard) = tracing_appender::non_blocking(daily_appender);
    let error_appender = tracing_appender::rolling::never("./logs", "error.log");
    let (error_no_blocking, _guard) = tracing_appender::non_blocking(error_appender);
    let offset = UtcOffset::current_local_offset().expect("should not get local offset!");
    let timer = OffsetTime::new(offset, time::format_description::well_known::Rfc3339);
    let layers = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(timer.clone())
                .with_writer(std::io::stdout.with_max_level(Level::INFO))
                .pretty(),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(daily_no_blocking.with_max_level(Level::INFO))
                .with_ansi(false)
                .with_timer(timer.clone()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(error_no_blocking.with_max_level(Level::ERROR))
                .with_ansi(false)
                .with_timer(timer),
        );
    #[cfg(feature = "console")]
    {
        let console_layer = console_subscriber::spawn();
        layers.with(console_layer).init();
    }
    #[cfg(not(feature = "console"))]
    layers.init();
    info!("Print server config: \n {}", &config);
    info!("Iot server start at: {}", &config.bind_address.clone());
    cli.execute(config).await
}
