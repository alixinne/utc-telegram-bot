#[macro_use]
extern crate tracing;

use structopt::StructOpt;
use thiserror::Error;
use tracing_subscriber::util::SubscriberInitExt;

mod converter;
#[cfg(feature = "renderer")]
mod generate_images;
mod manifest;
#[cfg(feature = "run")]
mod run;

#[derive(StructOpt)]
#[structopt()]
pub enum Command {
    #[cfg(feature = "renderer")]
    /// Generate thumbnails for the inline query menu
    GenerateImages(generate_images::GenerateImagesOpts),
    #[cfg(feature = "run")]
    /// Run the bot daemon
    Run(run::RunOpts),
}

#[derive(StructOpt)]
pub struct Opt {
    /// Increase logging verbosity
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u32,
    /// Command to run
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[cfg(feature = "renderer")]
    #[error(transparent)]
    Renderer(#[from] generate_images::GenerateImagesError),
    #[cfg(feature = "run")]
    #[error(transparent)]
    Run(#[from] run::RunError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[paw::main]
#[tokio::main(worker_threads = 2)]
pub async fn main(opt: Opt) -> Result<(), CliError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("UTC_TELEGRAM_BOT_LOG").unwrap_or_else(
                |_| {
                    tracing_subscriber::EnvFilter::from_default_env().add_directive(
                        match opt.verbose {
                            0 => "utc_telegram_bot=warn",
                            1 => "utc_telegram_bot=info",
                            2 => "utc_telegram_bot=debug",
                            _ => "utc_telegram_bot=trace",
                        }
                        .parse()
                        .unwrap(),
                    )
                },
            ),
        )
        .without_time()
        .finish()
        .init();

    match opt.command {
        #[cfg(feature = "renderer")]
        Command::GenerateImages(generate_image_opts) => {
            generate_images::generate_images(generate_image_opts).await?
        }
        #[cfg(feature = "run")]
        Command::Run(run_opts) => run::run(run_opts).await?,
    }

    Ok(())
}
