use crate::converter::Map;

mod resvg;
mod cairo;

pub trait Renderer {
    fn render_image(&self, map: &Map) -> Vec<u8>;
}

pub fn new_resvg() -> impl Renderer {
    resvg::Renderer::new()
}

pub fn new_cairo() -> impl Renderer {
    cairo::Renderer::new()
}
