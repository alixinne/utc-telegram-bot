use super::{Transform, Transformer};

#[derive(Debug, Clone)]
pub struct Spongebob {}

impl Spongebob {
    pub fn new() -> Self {
        Self {}
    }
}

enum CurrentState {
    Lowercase,
    Uppercase,
}

struct SpongebobTransformer {
    current_state: CurrentState,
}

impl Default for SpongebobTransformer {
    fn default() -> Self {
        Self {
            current_state: CurrentState::Lowercase,
        }
    }
}

impl Transformer<'_> for SpongebobTransformer {
    fn transform_chr(&mut self, src: char, dest: &mut String) {
        match src {
            'l' | 'L' => {
                self.current_state = CurrentState::Uppercase;
                dest.push('L');
            }
            'i' | 'I' => {
                self.current_state = CurrentState::Lowercase;
                dest.push('i');
            }
            c if c.is_alphabetic() => match self.current_state {
                CurrentState::Uppercase => {
                    dest.extend(c.to_lowercase());
                    self.current_state = CurrentState::Lowercase;
                }
                CurrentState::Lowercase => {
                    dest.extend(c.to_uppercase());
                    self.current_state = CurrentState::Uppercase;
                }
            },
            c => {
                dest.push(c);
            }
        }
    }
}

impl Transform for Spongebob {
    fn get_transfomer(&'_ self) -> Box<dyn Transformer<'_> + '_> {
        Box::new(SpongebobTransformer::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_alpha() {
        let t = Spongebob::new();
        assert_eq!("HeLLo", &t.map_string("hello"));
    }

    #[test]
    fn test_non_alpha() {
        let t = Spongebob::new();
        assert_eq!("HeLLo WoRLd!", &t.map_string("hello world!"));
    }
}
