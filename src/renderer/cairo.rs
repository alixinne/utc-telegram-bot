use crate::converter::Map;

pub struct Renderer {
}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    fn render_map_image(&self, c: char) -> Vec<u8> {
        let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 128, 128).unwrap();
        let context = cairo::Context::new(&surface);

        context.set_source_rgb(0.95, 0.95, 0.95);
        context.paint();

        context.set_source_rgb(0.01, 0.01, 0.01);

        let mut desc = pango::FontDescription::from_string("DejaVu Serif");
        desc.set_size(1024 * 72);

        let layout = pangocairo::create_layout(&context).unwrap();
        layout.set_font_description(Some(&desc));
        layout.set_text(&c.to_string());
        layout.set_alignment(pango::Alignment::Center);

        let sz = layout.get_pixel_size();
        context.move_to(64.0 - sz.0 as f64 / 2.0, 64.0 - sz.1 as f64 / 2.0);
        pangocairo::show_layout(&context, &layout);

        drop(context);

        // Encode it as JPEG
        // TODO: DO not copy here
        let image = image::RgbaImage::from_raw(128, 128, (&*surface.get_data().unwrap()).to_vec()).unwrap();

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
