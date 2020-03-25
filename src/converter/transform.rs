pub trait Transformer<'a> {
    fn transform_chr(&mut self, src: char, dest: &mut String);
}

pub trait Transform: std::fmt::Debug {
    fn get_transfomer(&'_ self) -> Box<dyn Transformer + '_>;

    fn map_string(&self, src: &str) -> String {
        // First, look for ranges to map
        struct Range {
            start: usize,
            end: usize,
        }

        enum State {
            NotMapping,
            OpeningStar,
            Mapping {
                start: usize,
                last_char_whitespace: bool,
            }
        }

        let mut ranges = Vec::new();
        let mut state = State::NotMapping;

        for (idx, c) in src.char_indices() {
            match state {
                State::NotMapping => {
                    if c == '*' {
                        state = State::OpeningStar;
                    }
                },
                State::OpeningStar => {
                    if c.is_whitespace() {
                        // Not a star followed by something
                        state = State::NotMapping;
                    } else {
                        // Star followed by something, start mapping
                        state = State::Mapping {
                            start: idx,
                            last_char_whitespace: false
                        };
                    }
                },
                State::Mapping { start, last_char_whitespace } => {
                    if !last_char_whitespace && c == '*' {
                        // Star following non-whitespace
                        ranges.push(Range {
                            start,
                            end: idx,
                        });

                        state = State::NotMapping;
                    } else {
                        state = State::Mapping {
                            start,
                            last_char_whitespace: c.is_whitespace(),
                        };
                    }
                }
            }
        }

        // Now map actual ranges
        let mut transformer = self.get_transfomer();
        let mut result = String::with_capacity(src.len() * 2);

        if ranges.is_empty() {
            // No ranges, map everything
            for c in src.chars() {
                transformer.transform_chr(c, &mut result);
            }
        } else {
            let mut range_it = ranges.iter();
            let mut current_range = range_it.next();

            for (idx, c) in src.char_indices() {
                if let Some(range) = &current_range {
                    if idx == range.start - 1 {
                        // Do not push mapping open star
                    } else if idx >= range.start && idx < range.end {
                        // Inside mapping, transform char
                        transformer.transform_chr(c, &mut result);
                    } else if idx == range.end {
                        // Do not push mapping closing star
                        // Switch to next mapping
                        current_range = range_it.next();
                    } else {
                        // Not yet in mapping
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            }
        }

        result
    }
}
