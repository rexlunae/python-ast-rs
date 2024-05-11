use pyo3::prelude::*;

/// A wrapper for the Python ast.dump function. This is a convenience function for dumping the AST
/// to the terminal.
pub fn dump(o: &PyAny, indent: Option<u8>) -> PyResult<String> {
    let pymodule_code = include_str!("__init__.py");

    Python::with_gil(|py| -> PyResult<String> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "dump.py", "parser")?;
        let t = pymodule.getattr("dump")?;
        assert!(t.is_callable());
        let args = (o, indent);

        Ok(t.call1(args)?.extract()?)
    })
}
