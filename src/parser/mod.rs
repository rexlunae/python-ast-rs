use crate::{
    tree::Module,
    ast_dump,
};

use pyo3::prelude::*;

/// Takes a string of bytes and returns the Python-tokenized version of it.
pub fn parse<'a>(input: &'a str, filename: &str) -> PyResult<Module> {

    let pymodule_code = include_str!("__init__.py");

    Python::with_gil(|py| -> PyResult<Module> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "parser.py", "parser")?;
        let t = pymodule.getattr("parse")?;
        assert!(t.is_callable());
        let args = (input, filename);

        let py_tree = t.call1(args)?;
        log::debug!("py_tree: {}", ast_dump(py_tree, Some(4))?);

        let tree: Module = py_tree.extract()?;

        Ok(tree)
    })
}
