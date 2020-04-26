use super::{CharMap, Transform, Transformer};

#[derive(Debug, Clone)]
pub struct Map {
    char_map: CharMap,
}

impl Map {
    pub fn new(char_map: CharMap) -> Self {
        Self { char_map }
    }
}

struct MapTransformer<'a> {
    char_map: &'a CharMap,
}

impl<'a> Transformer<'a> for MapTransformer<'a> {
    fn transform_chr(&mut self, src: char, dest: &mut String) {
        self.char_map.map_chr(src, dest)
    }
}

impl Transform for Map {
    fn get_transfomer(&'_ self, _: &str) -> Box<dyn Transformer<'_> + '_> {
        Box::new(MapTransformer {
            char_map: &self.char_map,
        })
    }
}
