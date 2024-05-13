use std::borrow::Borrow;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError<S: Into<String> + Clone + Ord + Borrow<S>> {
    #[error("searching path {0} failed")]
    PathNotFound(S),
    #[error("Not yet implemented: {0}")]
    NotYetImplemented(S),
    #[error("Unknown type {0}")]
    UnknownType(S),
}

pub type Result<T, S> = std::result::Result<T, CodeGenError<S>>;
