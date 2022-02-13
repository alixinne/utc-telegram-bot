use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;
use thiserror::Error;

use crate::converter;
use crate::manifest::{ImageManifest, Manifest, ManifestError};

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
    #[error(transparent)]
    Manifest(#[from] ManifestError),
}

pub async fn generate_images(opt: GenerateImagesOpts) -> Result<(), GenerateImagesError> {
    // Create output directory
    if !opt.output.is_dir() {
        fs::create_dir_all(&opt.output)?;
    }

    // Asset manifest
    let mut manifest = Manifest::default();

    let renderer = renderer::new_cairo();
    for transform in converter::TransformList::new().transforms() {
        debug!("rendering image for {}", transform.full_name);

        let file_name = transform.short_name.clone() + ".jpg";

        // Write image file
        let image = renderer.render_image(transform.as_ref());
        std::fs::write(opt.output.join(PathBuf::from(&file_name)), &image[..]).unwrap();

        // Add manifest entry
        manifest
            .images
            .insert(file_name.clone(), ImageManifest::new(&image[..]));
    }

    manifest.write(opt.output.join(".manifest.json"))?;

    Ok(())
}
