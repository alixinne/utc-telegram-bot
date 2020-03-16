use failure::Fail;

#[derive(Fail, Debug, Eq, PartialEq, Clone)]
pub enum MapError {
    #[fail(display = "the character map {} doesn't exist", 0)]
    MapNotFound(String),
}
