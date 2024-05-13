use crate::{dump, Module, Name, *};

use pyo3::prelude::*;

/// Takes a string of Python code and emits a Python struct that represents the AST.
fn parse_to_py(
    input: impl AsRef<str>,
    filename: impl AsRef<str>,
    py: Python<'_>,
) -> PyResult<PyObject> {
    let pymodule_code = include_str!("__init__.py");

    // We want to call tokenize.tokenize from Python.
    let pymodule = PyModule::from_code(py, pymodule_code, "__init__.py", "parser")?;
    let t = pymodule.getattr("parse")?;
    assert!(t.is_callable());
    let args = (input.as_ref(), filename.as_ref());

    let py_tree = t.call1(args)?;
    log::debug!("py_tree: {}", dump(py_tree, Some(4))?);

    Ok(py_tree.into())
}

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
    let filename = filename.as_ref();
    let mut module: Module = Python::with_gil(|py| {
        let py_tree = parse_to_py(input, filename, py)?;
        py_tree.extract(py)
    })?;
    module.filename = Some(filename.into());

    if let Some(name_str) = filename.strip_suffix(".py") {
        module.name =
            Some(Name::try_from(name_str).unwrap_or_else(|_| panic!("Invalid name {}", name_str)));
    }

    println!("module: {:#?}", module);
    for item in module.__dir__() {
        println!("module.__dir__: {:#?}", item.as_ref());
    }
    Ok(module)
}
