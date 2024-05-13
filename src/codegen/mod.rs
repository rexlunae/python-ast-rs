//! Code generation for Python ASTs.

use std::fmt::Debug;

pub mod python_options;
pub use python_options::*;

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
