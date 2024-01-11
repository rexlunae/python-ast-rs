use pyo3::prelude::*;

/// Accepts any Python object and dumps it using the Python ast module.
pub fn ast_dump(o: &PyAny, indent: Option<u8>) -> PyResult<String> {

    let pymodule_code = include_str!("__init__.py");

    Python::with_gil(|py| -> PyResult<String> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "ast_dump.py", "parser")?;
        let t = pymodule.getattr("ast_dump")?;
        assert!(t.is_callable());
        let args = (o, indent);

        Ok(t.call1(args)?.extract()?)
    })

}
