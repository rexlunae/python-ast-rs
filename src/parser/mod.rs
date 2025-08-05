use crate::{dump, Module, Name, SourceLocation, Error, Result as CrateResult, *};

use pyo3::prelude::*;
use std::ffi::CString;

use std::path::MAIN_SEPARATOR;

/// Takes a string of Python code and emits a Python struct that represents the AST.
fn parse_to_py(
    input: impl AsRef<str>,
    filename: impl AsRef<str>,
    py: Python<'_>,
) -> PyResult<PyObject> {
    let pymodule_code = include_str!("__init__.py");

    // We want to call tokenize.tokenize from Python.
    let code_cstr = CString::new(pymodule_code)?;
    let pymodule = PyModule::from_code(py, &code_cstr, c"__init__.py", c"parser")?;
    let t = pymodule.getattr("parse")?;
    assert!(t.is_callable());
    let args = (input.as_ref(), filename.as_ref());

    let py_tree = t.call1(args)?;
    log::debug!("py_tree: {}", dump(&py_tree, Some(4))?);

    Ok(py_tree.into())
}

/// Parses Python code and returns the AST as a Module with improved error handling.
/// 
/// This function accepts any type that can be converted to a string reference,
/// making it flexible for different input types. It provides detailed error information
/// including file location and helpful guidance when parsing fails.
/// 
/// # Arguments
/// * `input` - The Python source code to parse
/// * `filename` - The filename to associate with the parsed code
/// 
/// # Returns
/// * `CrateResult<Module>` - The parsed AST module or a detailed error with location info
/// 
/// # Examples
/// ```rust
/// use python_ast::parse_enhanced;
/// 
/// let code = "x = 1 + 2";
/// let module = parse_enhanced(code, "example.py").unwrap();
/// ```
pub fn parse_enhanced(input: impl AsRef<str>, filename: impl AsRef<str>) -> CrateResult<Module> {
    let filename = filename.as_ref();
    let input_str = input.as_ref();
    let location = SourceLocation::new(filename);
    
    // Empty files are valid in Python (they create empty modules), so we don't treat them as errors
    
    let mut module: Module = Python::with_gil(|py| {
        let py_tree = parse_to_py(input_str, filename, py)
            .map_err(|py_err| {
                // Convert PyO3 errors to our more detailed error format
                let error_msg = format!("Python parsing failed: {}", py_err);
                let help_msg = if error_msg.contains("SyntaxError") {
                    "Check your Python syntax. Common issues include missing colons, incorrect indentation, or unclosed brackets."
                } else if error_msg.contains("IndentationError") {
                    "Fix indentation issues. Python requires consistent indentation (use either spaces or tabs, not both)."
                } else {
                    "Ensure the input contains valid Python code. Check for syntax errors or unsupported constructs."
                };
                
                Error::parsing_error(location.clone(), error_msg, help_msg)
            })?;
            
        py_tree.extract(py)
            .map_err(|py_err| {
                Error::parsing_error(
                    location.clone(),
                    format!("Failed to extract AST: {}", py_err),
                    "The Python code was parsed but could not be converted to our AST format. This may indicate unsupported Python features."
                )
            })
    })?;
    
    module.filename = Some(filename.into());

    if let Some(name_str) = filename.replace(MAIN_SEPARATOR, "__").strip_suffix(".py") {
        module.name = Some(Name::try_from(name_str).map_err(|_| {
            Error::parsing_error(
                location,
                format!("Invalid module name derived from filename: '{}'", name_str),
                "Use a valid Python identifier for the filename (without special characters except underscores)."
            )
        })?);
    }

    log::debug!("module: {:#?}", module);
    for item in module.__dir__() {
        log::debug!("module.__dir__: {:#?}", item.as_ref());
    }
    Ok(module)
}

/// Parses Python code and returns the AST as a Module (backward compatible version).
/// 
/// This is the original parse function that returns PyResult for backward compatibility.
/// For better error messages with location information, use `parse_enhanced` instead.
/// 
/// # Arguments
/// * `input` - The Python source code to parse
/// * `filename` - The filename to associate with the parsed code
/// 
/// # Returns
/// * `PyResult<Module>` - The parsed AST module or a PyO3 error
/// 
/// # Examples
/// ```rust
/// use python_ast::parse;
/// 
/// let code = "x = 1 + 2";
/// let module = parse(code, "example.py").unwrap();
/// ```
pub fn parse(input: impl AsRef<str>, filename: impl AsRef<str>) -> PyResult<Module> {
    // Use the enhanced version but convert the error type for backward compatibility
    parse_enhanced(input, filename).map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let code = "1 + 2";
        let result = parse(code, "test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert!(module.filename.is_some());
        assert_eq!(module.filename.as_ref().unwrap(), "test.py");
        assert!(!module.raw.body.is_empty());
    }

    #[test]
    fn test_parse_function_definition() {
        let code = r#"
def hello_world():
    return "Hello, World!"
"#;
        let result = parse(code, "function_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 1);
    }

    #[test]
    fn test_parse_class_definition() {
        let code = r#"
class TestClass:
    def __init__(self):
        self.value = 42
        
    def get_value(self):
        return self.value
"#;
        let result = parse(code, "class_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 1);
    }

    #[test]
    fn test_parse_import_statements() {
        let code = r#"
import os
import sys
from collections import defaultdict
from typing import List, Dict
"#;
        let result = parse(code, "import_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 4);
    }

    #[test]
    fn test_parse_complex_expressions() {
        let code = r#"
result = [x**2 for x in range(10) if x % 2 == 0]
data = {"key": value for key, value in items.items()}
condition = (a > b) and (c < d) or (e == f)
"#;
        let result = parse(code, "expressions_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 3);
    }

    #[test]
    fn test_parse_control_flow() {
        let code = r#"
if condition:
    for i in range(10):
        if i % 2 == 0:
            continue
        else:
            break
else:
    while True:
        try:
            do_something()
        except Exception as e:
            handle_error(e)
        finally:
            cleanup()
"#;
        let result = parse(code, "control_flow_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 1);
    }

    #[test]
    fn test_parse_async_code() {
        let code = r#"
async def async_function():
    async with async_context():
        result = await some_async_operation()
        async for item in async_iterator:
            yield item
"#;
        let result = parse(code, "async_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 1);
    }

    #[test]
    fn test_parse_decorators() {
        let code = r#"
@decorator
@another_decorator(arg1, arg2)
def decorated_function():
    pass

@property
def getter(self):
    return self._value
"#;
        let result = parse(code, "decorators_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 2);
    }

    #[test]
    fn test_parse_invalid_syntax() {
        let code = "def invalid_function(";  // Missing closing parenthesis
        let result = parse(code, "invalid.py");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_file() {
        let code = "";
        let result = parse(code, "empty.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert!(module.raw.body.is_empty());
    }

    #[test]
    fn test_parse_comments_and_docstrings() {
        let code = r#"
"""Module docstring"""
# This is a comment
def function_with_docstring():
    """Function docstring"""
    pass  # Another comment
"#;
        let result = parse(code, "comments_test.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert_eq!(module.raw.body.len(), 2); // Docstring + function
    }

    #[test]
    fn test_module_name_generation() {
        let result = parse("x = 1", "some_file.py");
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert!(module.name.is_some());
        assert_eq!(module.name.unwrap().id, "some_file");
    }

    #[test]
    fn test_module_name_with_path_separators() {
        let code = "x = 1";
        let filename = format!("path{}to{}module.py", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR);
        let result = parse(code, &filename);
        assert!(result.is_ok());
        
        let module = result.unwrap();
        assert!(module.name.is_some());
        assert_eq!(module.name.unwrap().id, "path__to__module");
    }
}
