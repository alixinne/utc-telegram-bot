use std::path::PathBuf;

use structopt::StructOpt;

use crate::{converter, renderer, renderer::Renderer};

#[derive(StructOpt)]
#[structopt()]
pub struct GenerateImagesOpts {
    #[structopt(short, long, default_value = ".")]
    output: PathBuf,
}

pub async fn generate_images(opt: GenerateImagesOpts) -> Result<(), failure::Error> {
    let renderer = renderer::new_cairo();
    for map in converter::MapList::new().maps() {
        debug!("rendering image for {}", map.full_name);

        let image = renderer.render_image(map);
        std::fs::write(
            opt.output
                .clone()
                .join(PathBuf::from(map.short_name.clone() + ".jpg")),
            &image[..],
        )
        .unwrap();
    }

    Ok(())
}
