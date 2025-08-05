/// Generic utilities for parsing Python AST objects with consistent error handling.

use pyo3::{Bound, PyAny, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use crate::{Node, dump};

/// Generic function for extracting Python operator types with consistent error handling.
pub fn extract_operator_type<T>(
    ob: &Bound<PyAny>,
    attr_name: &str,
    context: &str,
) -> PyResult<String>
where
    T: std::fmt::Debug,
{
    let op = ob.getattr(attr_name).map_err(|_| {
        pyo3::exceptions::PyAttributeError::new_err(
            ob.error_message("<unknown>", format!("error getting {}", context))
        )
    })?;

    let op_type = op.get_type().name().map_err(|_| {
        pyo3::exceptions::PyTypeError::new_err(
            ob.error_message(
                "<unknown>",
                format!("extracting type name for {}", context),
            )
        )
    })?;

    op_type.extract()
}

/// Generic function for extracting operands from binary operations.
pub fn extract_binary_operands<L, R>(
    ob: &Bound<PyAny>,
    left_attr: &str,
    right_attr: &str,
    context: &str,
) -> PyResult<(L, R)>
where
    L: for<'a> pyo3::FromPyObject<'a>,
    R: for<'a> pyo3::FromPyObject<'a>,
{
    let left = ob.getattr(left_attr).map_err(|_| {
        pyo3::exceptions::PyAttributeError::new_err(
            ob.error_message("<unknown>", format!("error getting {} left operand", context))
        )
    })?;

    let right = ob.getattr(right_attr).map_err(|_| {
        pyo3::exceptions::PyAttributeError::new_err(
            ob.error_message("<unknown>", format!("error getting {} right operand", context))
        )
    })?;

    let left = left.extract().map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(
            format!("Failed to extract {} left operand: {}", context, e)
        )
    })?;

    let right = right.extract().map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(
            format!("Failed to extract {} right operand: {}", context, e)
        )
    })?;

    Ok((left, right))
}

/// Generic function for extracting lists of items with error handling.
pub fn extract_list<T>(
    ob: &Bound<PyAny>,
    attr_name: &str,
    context: &str,
) -> PyResult<Vec<T>>
where
    T: for<'a> pyo3::FromPyObject<'a>,
{
    let list_obj = ob.getattr(attr_name).map_err(|_| {
        pyo3::exceptions::PyAttributeError::new_err(
            ob.error_message("<unknown>", format!("error getting {} list", context))
        )
    })?;

    list_obj.extract().map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(
            format!("Failed to extract {} list: {}", context, e)
        )
    })
}

/// Generic function to safely extract optional attributes.
pub fn extract_optional<T>(
    ob: &Bound<PyAny>,
    attr_name: &str,
) -> Option<T>
where
    T: for<'a> pyo3::FromPyObject<'a>,
{
    ob.getattr(attr_name)
        .ok()
        .and_then(|attr| attr.extract().ok())
}

/// Generic function to extract position information from AST nodes.
pub fn extract_position_info(ob: &Bound<PyAny>) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    (
        extract_optional(ob, "lineno"),
        extract_optional(ob, "col_offset"),
        extract_optional(ob, "end_lineno"),
        extract_optional(ob, "end_col_offset"),
    )
}

/// Trait for types that can be extracted from Python with improved error messages.
pub trait ExtractFromPython<'a>: Sized {
    /// Extract from Python object with context for better error messages.
    fn extract_with_context(ob: &Bound<'a, PyAny>, context: &str) -> PyResult<Self>;
}

impl<'a, T> ExtractFromPython<'a> for T
where
    T: pyo3::FromPyObject<'a>,
{
    fn extract_with_context(ob: &Bound<'a, PyAny>, context: &str) -> PyResult<Self> {
        ob.extract().map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(
                format!("Failed to extract {} from Python: {} (object: {})", 
                    context, e, dump(ob, None).unwrap_or_else(|_| "unknown".to_string()))
            )
        })
    }
}

/// Utility function for consistent logging during Python object extraction.
pub fn log_extraction(ob: &Bound<PyAny>, context: &str) {
    if log::log_enabled!(log::Level::Debug) {
        match dump(ob, None) {
            Ok(dump_str) => log::debug!("Extracting {}: {}", context, dump_str),
            Err(_) => log::debug!("Extracting {} (dump failed)", context),
        }
    }
}

/// Helper function to create standardized error messages for failed extractions.
pub fn extraction_error(context: &str, details: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(
        format!("Failed to extract {}: {}", context, details)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::Python;

    #[test]
    fn test_extract_optional() {
        Python::with_gil(|py| {
            use std::ffi::CString;
            // Create a simple Python integer which has some attributes
            let code = CString::new("42").unwrap();
            let obj = py.eval(&code, None, None).unwrap();
            
            // Try to extract something that probably won't exist 
            let missing: Option<String> = extract_optional(&obj, "missing_attr");
            assert_eq!(missing, None);
            
            // This test mainly checks that extract_optional doesn't panic
            // and properly returns None for missing attributes
        });
    }

    #[test]
    fn test_log_extraction() {
        Python::with_gil(|py| {
            use std::ffi::CString;
            let code = CString::new("42").unwrap();
            let obj = py.eval(&code, None, None).unwrap();
            
            // Should not panic
            log_extraction(&obj, "test object");
        });
    }

    #[test]
    fn test_extraction_error() {
        let error = extraction_error("test context", "test details");
        let error_string = format!("{}", error);
        assert!(error_string.contains("test context"));
        assert!(error_string.contains("test details"));
    }
}

/// Enhanced error handling utilities for parsing Python AST objects

/// Get an attribute from a Python object with better error messaging
pub fn get_attr_with_context<'a>(
    ob: &Bound<'a, PyAny>,
    attr_name: &str,
    context: &str,
) -> PyResult<Bound<'a, PyAny>> {
    ob.getattr(attr_name).map_err(|e| {
        let type_name = ob.get_type().name()
            .map(|s| s.to_string())
            .unwrap_or_else(|_| "<unknown>".to_string());
        let enhanced_msg = format!(
            "Failed to get attribute '{}' from {} ({}): {}",
            attr_name,
            context,
            type_name,
            e
        );
        pyo3::exceptions::PyAttributeError::new_err(enhanced_msg)
    })
}

/// Extract a value from PyAny with better error messaging
pub fn extract_with_context<'py, T>(
    value: &Bound<'py, PyAny>,
    context: &str,
    attr_name: &str,
) -> PyResult<T>
where
    T: pyo3::FromPyObject<'py>,
{
    value.extract().map_err(|e| {
        let type_name = value.get_type().name()
            .map(|s| s.to_string())
            .unwrap_or_else(|_| "<unknown>".to_string());
        let enhanced_msg = format!(
            "Failed to extract {} for attribute '{}': {}. Expected type: {}, got: {}",
            context,
            attr_name,
            e,
            std::any::type_name::<T>(),
            type_name
        );
        pyo3::exceptions::PyTypeError::new_err(enhanced_msg)
    })
}

/// Extract a required attribute with enhanced error messaging  
pub fn extract_required_attr<'py, T>(
    ob: &Bound<'py, PyAny>,
    attr_name: &str,
    context: &str,
) -> PyResult<T>
where
    T: pyo3::FromPyObject<'py>,
{
    let attr = get_attr_with_context(ob, attr_name, context)?;
    extract_with_context(&attr, context, attr_name)
}