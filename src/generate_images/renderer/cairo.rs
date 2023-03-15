use crate::converter::Transform;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    fn render_map_image(&self, c: &str) -> Vec<u8> {
        let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 128, 128).unwrap();
        let context = cairo::Context::new(&surface).expect("failed to create cairo context");

        context.set_source_rgb(0.95, 0.95, 0.95);
        context.paint().expect("paint failed");

        context.set_source_rgb(0.01, 0.01, 0.01);

        let mut desc = pango::FontDescription::from_string("DejaVu Serif");
        desc.set_size(1024 * 48);

        let layout = pangocairo::create_layout(&context);
        layout.set_font_description(Some(&desc));
        layout.set_text(c);
        layout.set_alignment(pango::Alignment::Center);

        let sz = layout.pixel_size();
        context.move_to(64.0 - sz.0 as f64 / 2.0, 64.0 - sz.1 as f64 / 2.0);
        pangocairo::show_layout(&context, &layout);

        drop(context);

        // Encode it as JPEG
        // TODO: DO not copy here
        let image =
            image::RgbaImage::from_raw(128, 128, (*surface.data().unwrap()).to_vec()).unwrap();

        let mut cursor = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(image)
            .write_to(&mut cursor, image::ImageOutputFormat::Jpeg(100))
            .unwrap();

        cursor.into_inner()
    }
}

impl super::Renderer for Renderer {
    fn render_image(&self, transform: &dyn Transform) -> Vec<u8> {
        self.render_map_image(&transform.map_string("ab"))
    }
}
