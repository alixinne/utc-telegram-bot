#[macro_use]
extern crate log;

mod cli;
mod converter;
#[cfg(feature = "renderer")]
mod renderer;

#[paw::main]
#[tokio::main]
async fn main(opt: cli::Opt) -> Result<(), failure::Error> {
    // Initialize logger
    env_logger::init();
    Ok(cli::dispatch(opt).await?)
}
