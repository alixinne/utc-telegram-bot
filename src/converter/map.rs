use std::fmt;

use heck::SnakeCase;

use super::CharMap;

#[derive(Clone)]
pub struct Map {
    pub short_name: String,
    pub name: String,
    pub full_name: String,
    pub idx: usize,

    pub char_map: CharMap,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("short_name", &self.short_name)
            .field("name", &self.name)
            .field("full_name", &self.full_name)
            .field("char_map", &self.char_map)
            .finish()
    }
}

impl Map {
    pub fn new(idx: usize, name: String, char_map: CharMap) -> Self {
        let full_name = name;
        let name = full_name
            .replace("(", "")
            .replace(")", "")
            .replace(" pseudoalphabet", "");
        let short_name = name.to_snake_case();

        Self {
            idx,
            short_name,
            name,
            full_name,
            char_map,
        }
    }
}

impl AsRef<CharMap> for Map {
    fn as_ref(&self) -> &CharMap {
        &self.char_map
    }
}
