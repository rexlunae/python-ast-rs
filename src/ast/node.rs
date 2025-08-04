//! A module for AST elements that represent a position in a source file. Implementing the Node trait allows
//! an ergonomic means of extracting line and column information from an item.

use pyo3::{Bound, PyAny, prelude::PyAnyMethods};

/// A trait for AST elements that represent a position in a source file. Implementing this trait allows
/// an ergonomic means of extracting line and column information from an item.
pub trait Node {
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
    fn error_message(&self, mod_name: impl AsRef<str>, message: impl AsRef<str>) -> String {
        format!(
            "{} {}:{:?}:{:?}",
            message.as_ref(),
            mod_name.as_ref(),
            self.lineno(),
            self.col_offset()
        )
    }
}

// Note: Direct PyAny implementation removed in favor of Bound<PyAny> implementation below

/// Generic helper function for extracting position attributes from PyAny objects.
fn extract_position_attr(obj: &Bound<PyAny>, attr_name: &str) -> Option<usize> {
    obj.getattr(attr_name)
        .ok()
        .and_then(|attr| attr.extract().ok())
}

impl<'py> Node for &Bound<'py, PyAny> {
    /// A method for getting the starting line number of the node. This may not exist for all node types.
    fn lineno(&self) -> Option<usize> {
        extract_position_attr(self, "lineno")
    }

    /// A method for getting the starting column of the node. This may not exist for all node types.
    fn col_offset(&self) -> Option<usize> {
        extract_position_attr(self, "col_offset")
    }

    /// A method for getting the ending line number of the node. This may not exist for all node types.
    fn end_lineno(&self) -> Option<usize> {
        extract_position_attr(self, "end_lineno")
    }

    /// A method for getting the ending column of the node. This may not exist for all node types.
    fn end_col_offset(&self) -> Option<usize> {
        extract_position_attr(self, "end_col_offset")
    }
}
