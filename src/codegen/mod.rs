use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet},
    default::Default,
    fmt::{Debug},
    fs::File,
    io::prelude::*,
    path::{Path, MAIN_SEPARATOR},
};

use pyo3::{PyAny, PyResult};

use crate::{sys_path, Scope};
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

/// The global context for Python compilation.
#[derive(Clone, Debug)]
pub struct PythonOptions {
    /// Python imports are mapped into a given namespace that can be changed.
    pub python_namespace: String,

    /// The default path we will search for Python modules.
    pub python_path: Vec<String>,

    /// Collects all of the things we need to compile imports[module][asnames]
    pub imports: BTreeMap<String, HashSet<String>>,

    pub scope: Scope,

    pub stdpython: String,
    pub with_std_python: bool,

    pub allow_unsafe: bool,
}

impl Default for PythonOptions {
    fn default() -> Self {
        Self {
            python_namespace: String::from("__python_namespace__"),
            // XXX: Remove unwrap.
            python_path: sys_path().unwrap(),
            imports: BTreeMap::new(),
            scope: Scope::default(),
            stdpython: "stdpython".to_string(),
            with_std_python: true,
            allow_unsafe: false,
        }
    }
}

/// Reexport the CodeGen from to_tokenstream
pub use to_tokenstream::CodeGen;

#[derive(Clone, Copy, Debug)]
pub enum CodeGenContext {
    Module,
    Class,
    Function,
}

/// A trait for AST elements that represent a position in a source file. Implementing this trait allows
/// an ergonomic means of extracting line and column information from an item.
pub trait Node<'a> {
    /// A method for getting the starting line number of the node. This may not exist for all node types.
    fn lineno(&self) -> Option<usize> {
        None
    }

    /// A method for getting the starting column of the node. This may not exist for all node types.
    fn col_offset(&self) -> Option<usize> {
        None
    }

    /// A method for getting the ending line number of the node. This may not exist for all node types.
    fn end_lineno(&self) -> Option<usize> {
        None
    }

    /// A method for getting the ending column of the node. This may not exist for all node types.
    fn end_col_offset(&self) -> Option<usize> {
        None
    }

    /// Generate an error message for the current code, adding line and column number.
    fn error_message(&self, mod_name: &'a str, message: &'a str) -> String {
        format!("{} {}:{:?}:{:?}", message, mod_name, self.lineno(), self.col_offset())
    }
}

// These will only work on objects of Python's ast library's nodes, but you can try them on anything.
impl<'a> Node<'a> for PyAny {
    /// A method for getting the starting line number of the node. This may not exist for all node types.
    fn lineno(&self) -> Option<usize> {
        let lineno = self.getattr("lineno");
        if let Ok(ln_any) = lineno {
            let ln: PyResult<usize> = ln_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }

    /// A method for getting the starting column of the node. This may not exist for all node types.
    fn col_offset(&self) -> Option<usize> {
        let col_offset = self.getattr("col_offset");
        if let Ok(offset_any) = col_offset {
            let ln: PyResult<usize> = offset_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }

    /// A method for getting the ending line number of the node. This may not exist for all node types.
    fn end_lineno(&self) -> Option<usize> {
        let lineno = self.getattr("end_lineno");
        if let Ok(ln_any) = lineno {
            let ln: PyResult<usize> = ln_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }

    /// A method for getting the ending column of the node. This may not exist for all node types.
    fn end_col_offset(&self) -> Option<usize> {
        let col_offset = self.getattr("end_col_offset");
        if let Ok(offset_any) = col_offset {
            let ln: PyResult<usize> = offset_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }
}