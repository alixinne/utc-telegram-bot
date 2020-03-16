use std::collections::HashMap;
use std::fmt;

use failure::Fail;
use fuzzy_matcher::FuzzyMatcher;
use heck::SnakeCase;

static RAW_MAP_STR: &str = include_str!("../alphabets.txt");
const CHARS_PER_MAP: usize = 126 - 33 + 1;

#[derive(Clone)]
pub struct CharMap {
    data: [char; CHARS_PER_MAP],
    insert_nonbreak: bool,
}

impl CharMap {
    pub fn new() -> Self {
        Self {
            data: ['\0'; CHARS_PER_MAP],
            insert_nonbreak: false,
        }
    }

    fn map_char_to_range(c: char) -> Option<usize> {
        if c.is_ascii() {
            let mut buf = [0; 1];
            let _ = c.encode_utf8(&mut buf[..]);

            if buf[0] >= 33 && buf[0] <= 126 {
                return Some(buf[0] as usize - 33);
            }
        }

        None
    }

    pub fn set_idx(&mut self, idx: usize, dst: char) {
        self.data[idx] = dst;
    }

    pub fn map_chr(&self, src: char) -> String {
        Self::map_char_to_range(src)
            .map(|idx| {
                if self.insert_nonbreak && self.data[idx] != src {
                    let mut s = String::with_capacity(2);
                    s.push(self.data[idx]);
                    s.push('\u{00A0}');
                    s
                } else {
                    self.data[idx].to_string()
                }
            })
            .unwrap_or_else(|| src.to_string())
    }

    pub fn map_string(&self, src: &str) -> String {
        src.chars().map(|c| self.map_chr(c)).collect()
    }
}

impl fmt::Debug for CharMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

#[derive(Clone)]
pub struct Map {
    pub short_name: String,
    pub name: String,
    pub full_name: String,
    pub idx: usize,

    pub char_map: CharMap,
    pub image: Vec<u8>,
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

struct Renderer {
    backend: Box<dyn resvg::Render>,
}

impl Renderer {
    fn new() -> Self {
        let backend = resvg::default_backend();

        Self { backend }
    }

    fn render_map_image(&self, c: char) -> Vec<u8> {
        use resvg::prelude::*;

        let opt = Options::default();

        let svg_source = include_str!("../assets/char_thumb.svg").replace("X", &c.to_string());
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
            image: Vec::new(),
        }
    }

    fn render_image(&mut self, renderer: &Renderer) {
        self.image = renderer.render_map_image(self.char_map.map_chr('a').chars().next().unwrap());

        std::fs::write(self.short_name.clone() + ".jpg", &self.image[..]).unwrap();
    }
}

impl AsRef<CharMap> for Map {
    fn as_ref(&self) -> &CharMap {
        &self.char_map
    }
}

#[derive(Fail, Debug, Eq, PartialEq, Clone)]
pub enum MapError {
    #[fail(display = "the character map {} doesn't exist", 0)]
    MapNotFound(String),
}

type BotFuzzyMatcher = fuzzy_matcher::skim::SkimMatcherV2;

pub struct MapList {
    maps: HashMap<String, Map>,
    matcher: BotFuzzyMatcher,
}

pub struct FuzzyMatchResult<'a> {
    pub map: &'a Map,
    pub score: i64,
    pub result: String,
}

impl MapList {
    pub fn new() -> Self {
        let mut mp = HashMap::new();

        let mut current_line = CharMap::new();
        let mut current_name = String::with_capacity(80);

        enum State {
            ParseName,
            ParseMap { current_index: usize },
        }

        let mut current_state = State::ParseName;
        let mut current_map_idx = 0;

        for c in RAW_MAP_STR.chars() {
            match current_state {
                State::ParseName => match c {
                    '\t' => {
                        current_state = State::ParseMap { current_index: 0 };
                    }
                    other @ _ => {
                        current_name.push(other);
                    }
                },
                State::ParseMap { current_index } => match c {
                    '\r' => {}
                    '\n' => {
                        let mut map = Map::new(
                            current_map_idx,
                            std::mem::replace(&mut current_name, String::with_capacity(80)),
                            std::mem::replace(&mut current_line, CharMap::new()),
                        );

                        if !map.full_name.starts_with("#") {
                            if map.full_name.contains("Regional") {
                                map.char_map.insert_nonbreak = true;
                            }

                            mp.insert(map.short_name.clone(), map);
                        }

                        current_state = State::ParseName;
                        current_map_idx += 1;
                    }
                    other @ _ => {
                        current_line.set_idx(current_index, other);
                        current_state = State::ParseMap {
                            current_index: current_index + 1,
                        };
                    }
                },
            }
        }

        info!("loaded {} maps", mp.len());

        Self {
            maps: mp,
            matcher: BotFuzzyMatcher::default(),
        }
    }

    pub fn render_images(&mut self) {
        let len = self.maps.len();

        let renderer = Renderer::new();
        for (i, v) in self.maps.values_mut().enumerate() {
            debug!("rendering image {} of {} for {}", i + 1, len, v.short_name);
            v.render_image(&renderer);
        }
    }

    pub fn map_string(&self, map_name: &str, src: &str) -> Result<String, MapError> {
        self.maps
            .get(map_name)
            .map(|map| map.as_ref().map_string(src))
            .ok_or_else(|| MapError::MapNotFound(map_name.to_owned()))
    }

    pub fn get_fuzzy_matches<'a>(
        &'a self,
        partial_map_name: &str,
        src: &str,
    ) -> Vec<FuzzyMatchResult<'a>> {
        let mut v: Vec<_> = self
            .maps
            .iter()
            .filter_map(|(k, v)| {
                self.matcher
                    .fuzzy_match(k, partial_map_name)
                    .map(|score| FuzzyMatchResult {
                        map: v,
                        score,
                        result: v.as_ref().map_string(src),
                    })
            })
            .collect();

        v.sort_by_key(|r| r.score);
        v
    }

    pub fn get_all_matches<'a>(&'a self, src: &str) -> Vec<FuzzyMatchResult<'a>> {
        let mut v: Vec<_> = self
            .maps
            .iter()
            .map(|(k, v)| FuzzyMatchResult {
                map: v,
                score: v.idx as i64,
                result: v.as_ref().map_string(src),
            })
            .collect();

        v.sort_by_key(|r| r.score);
        v
    }
}

impl fmt::Debug for MapList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapList").field("maps", &self.maps).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_list_init() {
        println!("{:?}", MapList::new());
    }

    #[test]
    fn map_list_map_string_circled() {
        assert_eq!(
            Ok("Ⓗⓔⓛⓛⓞ, ⓦⓞⓡⓛⓓ!".to_owned()),
            MapList::new().map_string("circled", "Hello, world!")
        );
    }
}
