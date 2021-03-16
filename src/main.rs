#[macro_use]
extern crate log;

mod cli;
mod converter;
#[cfg(feature = "renderer")]
mod renderer;

#[paw::main]
#[tokio::main(flavor = "current_thread")]
async fn main(opt: cli::Opt) -> Result<(), cli::CliError> {
    // Initialize logger
    env_logger::init();
    Ok(cli::dispatch(opt).await?)
}
