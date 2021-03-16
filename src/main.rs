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
pub enum Opt {
    #[cfg(feature = "renderer")]
    GenerateImages(generate_images::GenerateImagesOpts),
    Run(run::RunOpts),
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
#[tokio::main(flavor = "current_thread")]
pub async fn main(opt: Opt) -> Result<(), CliError> {
    // Initialize logger
    env_logger::init();

    match opt {
        #[cfg(feature = "renderer")]
        Opt::GenerateImages(generate_image_opts) => {
            generate_images::generate_images(generate_image_opts).await?
        }
        Opt::Run(run_opts) => run::run(run_opts).await?,
    }

    Ok(())
}
