#![feature(strict_provenance)]
#[doc = include_str!("../README.md")]
pub mod ast;
pub use ast::*;

pub mod codegen;
pub use codegen::*;

pub mod isidentifier;
pub use isidentifier::*;

pub mod scope;
pub use scope::*;

pub mod symbols;
pub use symbols::*;

pub mod pytypes;

pub use pyo3::PyResult;

pub mod parser;
pub use parser::*;

pub mod datamodel;
pub use datamodel::*;
