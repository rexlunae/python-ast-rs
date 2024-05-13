use std::borrow::Borrow;

use thiserror::{Error};
use pyo3::PyErr;

#[derive(Error, Debug)]
pub enum CodeGenError<S: Into<String> + Clone + Ord + Borrow<S>> {
    #[error("searching path {0} failed")]
    PathNotFound(S),
    #[error("Not yet implemented: {0}")]
    NotYetImplemented(S),
    #[error("Unknown type {0}")]
    UnknownType(S),

    #[error("PyO3 Error: {0}")]
    #[from(PyErr)]
    Pyo3Error(PyErr),
}

pub type Result<T, S> = std::result::Result<T, CodeGenError<S>>;
