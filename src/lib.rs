#![feature(extend_one)]
#![feature(associated_type_bounds)]
extern crate proc_macro;

pub mod tree;
pub use tree::*;

pub mod codegen;
pub use codegen::*;

pub mod scope;
pub use scope::*;

pub mod symbols;
pub use symbols::*;

use pyo3::prelude::*;
use std::include_str;

pub mod pytypes;

pub use pyo3::PyResult;

/// Takes a string of bytes and returns the Python-tokenized version of it.
pub fn parse<'a>(input: &'a str, filename: &str) -> PyResult<tree::Module> {

    let pymodule_code = include_str!("parser.py");

    Python::with_gil(|py| -> PyResult<Module> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "parser.py", "parser")?;
        let t = pymodule.getattr("parse")?;
        assert!(t.is_callable());
        let args = (input, filename);

        let py_tree = t.call1(args)?;
        log::debug!("py_tree: {}", ast_dump(py_tree, Some(4))?);

        let tree: tree::Module = py_tree.extract()?;

        Ok(tree)
    })
}

/// Accepts any Python object and dumps it using the Python ast module.
pub fn ast_dump(o: &PyAny, indent: Option<u8>) -> PyResult<String> {

    let pymodule_code = include_str!("ast_dump.py");

    Python::with_gil(|py| -> PyResult<String> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "ast_dump.py", "parser")?;
        let t = pymodule.getattr("ast_dump")?;
        assert!(t.is_callable());
        let args = (o, indent);

        Ok(t.call1(args)?.extract()?)
    })

}

pub fn sys_path() -> PyResult<Vec<String>> {

    let pymodule_code = include_str!("path.py");

    Python::with_gil(|py| -> PyResult<Vec<String>> {
        let pymodule = PyModule::from_code(py, pymodule_code, "path.py", "path")?;
        let t = pymodule.getattr("path").expect("Reading path variable from interpretter");
        assert!(t.is_callable());
        let args = ();
        let paths: Vec<String> = t.call1(args)?.extract()?;

        Ok(paths)
    })
}
