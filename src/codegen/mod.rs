//! Code generation for Python ASTs.

use std::{
    borrow::Borrow,
    fmt::Debug,
};

pub mod python_options;
pub use python_options::*;

use thiserror::Error;

/// Code generation errors.
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

/// A type to track the context of code generation.
#[derive(Clone, Debug)]
pub enum CodeGenContext {
    Module(String),
    Class,
    Function,
    Async(Box<CodeGenContext>),
}
