use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;
use thiserror::Error;

use crate::converter;

mod renderer;
use renderer::Renderer;

#[derive(StructOpt)]
#[structopt()]
pub struct GenerateImagesOpts {
    #[structopt(short, long, default_value = "./public/images")]
    output: PathBuf,
}

#[derive(Error, Debug)]
pub enum GenerateImagesError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub async fn generate_images(opt: GenerateImagesOpts) -> Result<(), GenerateImagesError> {
    // Create output directory
    if !opt.output.is_dir() {
        fs::create_dir_all(&opt.output)?;
    }

    let renderer = renderer::new_cairo();
    for transform in converter::TransformList::new().transforms() {
        debug!("rendering image for {}", transform.full_name);

        let image = renderer.render_image(transform.as_ref());
        std::fs::write(
            opt.output
                .join(PathBuf::from(transform.short_name.clone() + ".jpg")),
            &image[..],
        )
        .unwrap();
    }

    Ok(())
}
