#[macro_use]
extern crate log;

use structopt::StructOpt;
use thiserror::Error;

mod converter;
#[cfg(feature = "renderer")]
mod generate_images;
mod run;

#[derive(StructOpt)]
#[structopt()]
pub enum Command {
    #[cfg(feature = "renderer")]
    /// Generate thumbnails for the inline query menu
    GenerateImages(generate_images::GenerateImagesOpts),
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
    #[error(transparent)]
    Run(#[from] run::RunError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[paw::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
pub async fn main(opt: Opt) -> Result<(), CliError> {
    // Initialize logger
    env_logger::Builder::from_env(
        env_logger::Env::new()
            .filter_or(
                "UTC_TELEGRAM_BOT_LOG",
                match opt.verbose {
                    0 => "warn",
                    1 => "info",
                    2 => "debug",
                    _ => "trace",
                },
            )
            .write_style("UTC_TELEGRAM_BOT_LOG_STYLE"),
    )
    .format_timestamp(None)
    .try_init()
    .ok();

    match opt.command {
        #[cfg(feature = "renderer")]
        Command::GenerateImages(generate_image_opts) => {
            generate_images::generate_images(generate_image_opts).await?
        }
        Command::Run(run_opts) => run::run(run_opts).await?,
    }

    Ok(())
}
