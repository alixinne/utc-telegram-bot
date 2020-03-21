use std::collections::HashMap;
use std::fmt;

use super::{CharMap, Map, TransformEntry, TransformError};

static RAW_MAP_STR: &str = include_str!("../../alphabets.txt");

pub struct TransformList {
    transforms: HashMap<String, TransformEntry>,
    matcher: BotFuzzyMatcher,
}

pub struct FuzzyMatchResult<'a> {
    pub transform: &'a TransformEntry,
    pub score: i64,
    pub result: String,
}

impl TransformList {
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
                        // Check if it needs the fix for combining chars
                        if current_name.contains("Regional") {
                            current_line.insert_nonbreak = true;
                        }

                        // Create map object
                        let map = Map::new(std::mem::replace(&mut current_line, CharMap::new()));

                        // Create transform entry
                        let entry = TransformEntry::new(
                            current_map_idx,
                            &current_name,
                            Box::new(map),
                        );

                        // current_name used by that point
                        current_name.clear();

                        if !entry.full_name.starts_with('#') {
                            mp.insert(entry.short_name.clone(), entry);
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

        info!("loaded {} transforms", mp.len());

        Self {
            transforms: mp,
            matcher: BotFuzzyMatcher::default(),
        }
    }

    pub fn transform_string(&self, map_name: &str, src: &str) -> Result<String, TransformError> {
        self.transforms
            .get(map_name)
            .map(|map| map.as_ref().map_string(src))
            .ok_or_else(|| TransformError::TransformNotFound(map_name.to_owned()))
    }

    pub fn get_fuzzy_matches<'a>(
        &'a self,
        partial_map_name: &str,
        src: &str,
    ) -> Vec<FuzzyMatchResult<'a>> {
        use fuzzy_matcher::FuzzyMatcher;

        let mut v: Vec<_> = self
            .transforms
            .iter()
            .filter_map(|(k, v)| {
                self.matcher
                    .fuzzy_match(k, partial_map_name)
                    .map(|score| FuzzyMatchResult {
                        transform: v,
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
            .transforms
            .iter()
            .map(|(_k, v)| FuzzyMatchResult {
                transform: v,
                score: v.idx as i64,
                result: v.as_ref().map_string(src),
            })
            .collect();

        v.sort_by_key(|r| r.score);
        v
    }

    #[allow(dead_code)]
    pub fn transforms(&self) -> impl Iterator<Item = &'_ TransformEntry> {
        self.transforms.values()
    }
}

impl fmt::Debug for TransformList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransformList")
            .field("transforms", &self.transforms)
            .finish()
    }
}

type BotFuzzyMatcher = fuzzy_matcher::skim::SkimMatcherV2;
