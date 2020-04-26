use crate::converter::Transform;

mod cairo;
mod resvg;

pub trait Renderer {
    fn render_image(&self, transform: &dyn Transform) -> Vec<u8>;
}

pub fn new_resvg() -> impl Renderer {
    resvg::Renderer::new()
}

pub fn new_cairo() -> impl Renderer {
    cairo::Renderer::new()
}
