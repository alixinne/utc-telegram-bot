use structopt::StructOpt;

#[cfg(feature = "renderer")]
mod generate_images;
mod run;

#[cfg(feature = "renderer")]
pub async fn dispatch(opt: Opt) -> Result<(), failure::Error> {
    Ok(match opt {
        Opt::Run(run_opts) => run::run(run_opts).await?,
        Opt::GenerateImages(generate_image_opts) => {
            generate_images::generate_images(generate_image_opts).await?
        }
    })
}

#[cfg(not(feature = "renderer"))]
pub async fn dispatch(opt: Opt) -> Result<(), failure::Error> {
    match opt {
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
