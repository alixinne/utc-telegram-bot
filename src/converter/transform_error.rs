use failure::Fail;

#[derive(Fail, Debug, Eq, PartialEq, Clone)]
pub enum TransformError {
    #[fail(display = "the transform {} doesn't exist", 0)]
    TransformNotFound(String),
}
