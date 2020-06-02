mod char_map;
pub use char_map::*;

mod map;
pub use map::*;

mod spongebob;
pub use spongebob::*;

mod spread;
pub use spread::*;

mod transform;
pub use transform::*;

mod transform_entry;
pub use transform_entry::*;

mod transform_error;
pub use transform_error::*;

mod transform_list;
pub use transform_list::*;

mod zalgo;
pub use zalgo::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_list_init() {
        println!("{:?}", TransformList::new());
    }

    #[test]
    fn transform_list_transform_string_circled() {
        assert_eq!(
            Ok("Ⓗⓔⓛⓛⓞ, ⓦⓞⓡⓛⓓ!".to_owned()),
            TransformList::new().transform_string("circled", "Hello, world!")
        );
    }
}
