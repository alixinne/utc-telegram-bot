use crate::converter::Map;

pub struct Renderer {
    backend: Box<dyn resvg::Render>,
}

impl Renderer {
    pub fn new() -> Self {
        let backend = resvg::default_backend();

        Self { backend }
    }

    fn render_map_image(&self, c: char) -> Vec<u8> {
        use resvg::prelude::*;

        let opt = Options::default();

        let svg_source = include_str!("../../assets/char_thumb.svg").replace("X", &c.to_string());
        let rtree = usvg::Tree::from_str(&svg_source, &usvg::Options::default()).unwrap();

        // Get raw RGBA data
        let rgba_data = self
            .backend
            .render_to_image(&rtree, &opt)
            .unwrap()
            .make_rgba_vec();

        // Encode it as JPEG
        let image = image::RgbaImage::from_raw(128, 128, rgba_data).unwrap();

        let mut encoded: Vec<u8> = Vec::new();
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut encoded, image::ImageOutputFormat::Jpeg(100))
            .unwrap();

        encoded
    }
}

impl super::Renderer for Renderer {
    fn render_image(&self, map: &Map) -> Vec<u8> {
        self.render_map_image(map.char_map.map_chr('a').chars().next().unwrap())
    }
}
