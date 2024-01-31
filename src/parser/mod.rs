use crate::{dump, Module};

use pyo3::prelude::*;

/// Takes a string of bytes and returns the Python-tokenized version of it.
/// use python_ast::parse;
///
/// ```Rust
/// fn read_python_file(input: std::path::Path) {
///    let py = read_to_string(input).unwrap();
///    let ast = parse(&py, "__main__").unwrap();
///
///    println!("{:?}", ast);
///}
/// ```
pub fn parse(input: impl AsRef<str>, filename: impl AsRef<str>) -> PyResult<Module> {

    let pymodule_code = include_str!("__init__.py");

    Python::with_gil(|py| -> PyResult<Module> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "__init__.py", "parser")?;
        let t = pymodule.getattr("parse")?;
        assert!(t.is_callable());
        let args = (input.as_ref(), filename.as_ref());

        let py_tree = t.call1(args)?;
        log::debug!("py_tree: {}", dump(py_tree, Some(4))?);

        let tree: Module = py_tree.extract()?;

        Ok(tree)
    })
}
