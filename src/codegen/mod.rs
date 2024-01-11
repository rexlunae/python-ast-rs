use std::{
    borrow::Borrow,
    fmt::Debug,
};

pub mod python_options;
pub use python_options::*;

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

/// Reexport the CodeGen from to_tokenstream
pub use to_tokenstream::CodeGen;

#[derive(Clone, Copy, Debug)]
pub enum CodeGenContext {
    Module,
    Class,
    Function,
}
