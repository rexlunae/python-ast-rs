use pyo3::{PyAny, PyResult};

/// A trait for AST elements that represent a position in a source file. Implementing this trait allows
/// an ergonomic means of extracting line and column information from an item.
pub trait Node<'a> {
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
    fn error_message(&self, mod_name: &'a str, message: &'a str) -> String {
        format!("{} {}:{:?}:{:?}", message, mod_name, self.lineno(), self.col_offset())
    }
}

// These will only work on objects of Python's ast library's nodes, but you can try them on anything.
impl<'a> Node<'a> for PyAny {
    /// A method for getting the starting line number of the node. This may not exist for all node types.
    fn lineno(&self) -> Option<usize> {
        let lineno = self.getattr("lineno");
        if let Ok(ln_any) = lineno {
            let ln: PyResult<usize> = ln_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }

    /// A method for getting the starting column of the node. This may not exist for all node types.
    fn col_offset(&self) -> Option<usize> {
        let col_offset = self.getattr("col_offset");
        if let Ok(offset_any) = col_offset {
            let ln: PyResult<usize> = offset_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }

    /// A method for getting the ending line number of the node. This may not exist for all node types.
    fn end_lineno(&self) -> Option<usize> {
        let lineno = self.getattr("end_lineno");
        if let Ok(ln_any) = lineno {
            let ln: PyResult<usize> = ln_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }

    /// A method for getting the ending column of the node. This may not exist for all node types.
    fn end_col_offset(&self) -> Option<usize> {
        let col_offset = self.getattr("end_col_offset");
        if let Ok(offset_any) = col_offset {
            let ln: PyResult<usize> = offset_any.extract();
            if let Ok(l) = ln {
                Some(l)
            } else {
                None
            }
        }
        else { None }
    }
}
