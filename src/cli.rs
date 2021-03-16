use structopt::StructOpt;
use thiserror::Error;

#[cfg(feature = "renderer")]
mod generate_images;
mod run;

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

pub async fn dispatch(opt: Opt) -> Result<(), CliError> {
    match opt {
        #[cfg(feature = "renderer")]
        Opt::GenerateImages(generate_image_opts) => {
            generate_images::generate_images(generate_image_opts).await?
        }
        Opt::Run(run_opts) => run::run(run_opts).await?,
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt()]
pub enum Opt {
    #[cfg(feature = "renderer")]
    GenerateImages(generate_images::GenerateImagesOpts),
    Run(run::RunOpts),
}
