pub trait Transformer<'a> {
    fn transform_chr(&mut self, src: char, dest: &mut String);
}

pub trait Transform: std::fmt::Debug {
    fn get_transfomer(&'_ self) -> Box<dyn Transformer + '_>;

    fn map_string(&self, src: &str) -> String {
        let mut transformer = self.get_transfomer();
        let mut result = String::with_capacity(src.len() * 2);

        for c in src.chars() {
            transformer.transform_chr(c, &mut result);
        }

        result
    }
}
