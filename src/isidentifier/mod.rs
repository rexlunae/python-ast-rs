//! This module uses Python to determine if a given string is a valid Python identifier or not.
//! See [here](https://docs.python.org/3/reference/lexical_analysis.html)
use pyo3::prelude::*;

/// Determines if a string is a valid Python idetifier, and returns a Python object wrapping a bool.
fn isidentifier_to_py(input: impl AsRef<str>, py: Python<'_>) -> PyResult<PyObject> {

    let pymodule_code = include_str!("__init__.py");

    // We want to call tokenize.tokenize from Python.
    let pymodule = PyModule::from_code(py, pymodule_code, "__init__.py", "isidentifier")?;
    let t = pymodule.getattr("isidentifier")?;
    assert!(t.is_callable());
    let args = (input.as_ref(),);

    let isidentifier = t.call1(args)?;

    Ok(isidentifier.into())
}


/// Takes a string of bytes and returns the Python-tokenized version of it.
/// use python_ast::parse;
///
/// ```Rust
/// isidentifier("alpha").expect('Should return Ok(true)')
/// isidentifier("0alpha").expect('Should return Ok(false)')
/// ```
fn isidentifier(input: impl AsRef<str>) -> PyResult<bool> {
    let isidentifier = Python::with_gil(|py| {
        let isidentifier = isidentifier_to_py(input, py)?;
        isidentifier.extract(py)
    })?;

    Ok(isidentifier)
}

/// Trait that determines if a string contains a valid identifer based on Python rules (which are broadly simiilar to Rust).
pub trait IsIdentifier: AsRef<str> {
    fn isidentifier(&self) -> PyResult<bool> {
        let s = self.as_ref();
        isidentifier(s)
    }
}

/// Blanket implementation for this trait.
impl<T: AsRef<str>> IsIdentifier for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_symbol_works() {
        assert_eq!(isidentifier("alpha").unwrap(), true)
    }

    #[test]
    fn good_symbol_works_as_method() {
        assert_eq!(isidentifier("alpha").unwrap(), true)
    }

    #[test]
    fn bad_symbol_works() {
        assert_eq!(isidentifier("0alpha").unwrap(), false)
    }

    #[test]
    fn bad_symbol_works_as_method() {
        assert_eq!("0alpha".isidentifier().unwrap(), false)
    }

}
