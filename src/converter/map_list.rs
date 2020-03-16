use std::collections::HashMap;
use std::fmt;

use super::{CharMap, Map, MapError};

static RAW_MAP_STR: &str = include_str!("../../alphabets.txt");

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
                    other => {
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

                        if !map.full_name.starts_with('#') {
                            if map.full_name.contains("Regional") {
                                map.char_map.insert_nonbreak = true;
                            }

                            mp.insert(map.short_name.clone(), map);
                        }

                        current_state = State::ParseName;
                        current_map_idx += 1;
                    }
                    other => {
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
        use fuzzy_matcher::FuzzyMatcher;

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
            .map(|(_k, v)| FuzzyMatchResult {
                map: v,
                score: v.idx as i64,
                result: v.as_ref().map_string(src),
            })
            .collect();

        v.sort_by_key(|r| r.score);
        v
    }

    #[allow(dead_code)]
    pub fn maps(&self) -> impl Iterator<Item = &'_ Map> {
        self.maps.values()
    }
}

impl fmt::Debug for MapList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapList").field("maps", &self.maps).finish()
    }
}

type BotFuzzyMatcher = fuzzy_matcher::skim::SkimMatcherV2;
