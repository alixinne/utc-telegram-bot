use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Clone)]
pub enum TransformError {
    #[error("the transform `{0}` doesn't exist")]
    TransformNotFound(String),
}
