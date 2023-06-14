use ::config::{Config, File};
use clap::Parser;
use time::UtcOffset;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod cli;
mod config;
mod panic_hook;
pub(crate) mod protocol;
mod router;
mod server;
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
    let offset = UtcOffset::current_local_offset().expect("should get local offset!");
    let timer = OffsetTime::new(offset, time::format_description::well_known::Rfc3339);
    let layers = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_timer(timer.clone()))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(daily_no_blocking)
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
        layers.with(console_layer)
    }
    layers.init();
    info!("Print server config: \n {}", &config);
    info!("Iot server started at: {}", &config.bind_address.clone());
    error!("Not implement error");
    Ok(())
}
