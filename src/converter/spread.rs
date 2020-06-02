use super::{Transform, Transformer};

#[derive(Default, Debug, Clone)]
pub struct Spread {}

impl Spread {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Debug, Clone)]
struct SpreadTransformer {}

impl SpreadTransformer {
    fn new() -> Self {
        Self::default()
    }
}

impl Transformer<'_> for SpreadTransformer {
    fn transform_chr(&mut self, src: char, dest: &mut String) {
        dest.push(src);
        dest.push(' ');
    }
}

impl Transform for Spread {
    fn get_transfomer(&'_ self, _src: &str) -> Box<dyn Transformer + '_> {
        Box::new(SpreadTransformer::new())
    }
}
