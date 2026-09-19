use std::sync::Arc;

use heck::ToSnakeCase;

use super::Transform;

#[derive(Debug, Clone)]
pub struct TransformEntry {
    pub short_name: String,
    #[expect(dead_code)]
    pub name: String,
    pub full_name: String,
    pub idx: usize,

    transform: Arc<dyn Transform + Send + Sync>,
}

impl TransformEntry {
    pub fn new(idx: usize, name: &str, transform: Arc<dyn Transform + Send + Sync>) -> Self {
        let full_name = name;
        let name = full_name
            .replace(['(', ')'], "")
            .replace(" pseudoalphabet", "");
        let short_name = name.to_snake_case();

        Self {
            short_name,
            name,
            full_name: full_name.to_owned(),
            idx,
            transform,
        }
    }
}

impl AsRef<dyn Transform + 'static> for TransformEntry {
    fn as_ref(&self) -> &(dyn Transform + 'static) {
        &*self.transform
    }
}
