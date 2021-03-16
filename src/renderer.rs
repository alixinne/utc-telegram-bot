use crate::converter::Transform;

mod cairo;

pub trait Renderer {
    fn render_image(&self, transform: &dyn Transform) -> Vec<u8>;
}

pub fn new_cairo() -> impl Renderer {
    cairo::Renderer::new()
}
