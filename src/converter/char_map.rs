use std::fmt;

const CHARS_PER_MAP: usize = 126 - 33 + 1;

#[derive(Clone)]
pub struct CharMap {
    data: [char; CHARS_PER_MAP],
    pub insert_nonbreak: bool,
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

    pub fn map_chr(&self, src: char, dest: &mut String) {
        if let Some(idx) = Self::map_char_to_range(src) {
            dest.push(self.data[idx]);

            if self.insert_nonbreak && self.data[idx] != src {
                dest.push('\u{00A0}');
            }
        } else {
            dest.push(src);
        }
    }
}

impl fmt::Debug for CharMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}
