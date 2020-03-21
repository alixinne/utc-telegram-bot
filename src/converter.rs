mod char_map;
pub use char_map::*;

mod map;
pub use map::*;

mod transform;
pub use transform::*;

mod transform_entry;
pub use transform_entry::*;

mod transform_error;
pub use transform_error::*;

mod transform_list;
pub use transform_list::*;

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
