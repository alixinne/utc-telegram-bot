mod char_map;
pub use char_map::*;

mod map;
pub use map::*;

mod map_error;
pub use map_error::*;

mod map_list;
pub use map_list::*;

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
