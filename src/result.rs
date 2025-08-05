
use pyo3::PyErr;
use thiserror::Error as E;

use crate::{BinOp, BoolOp, Compare, Expr, ExprType, StatementType, UnaryOp, PositionInfo};

/// Location information for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub filename: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub end_line: Option<usize>,
    pub end_column: Option<usize>,
}

impl SourceLocation {
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            line: None,
            column: None,
            end_line: None,
            end_column: None,
        }
    }

    pub fn with_position(
        filename: impl Into<String>,
        line: Option<usize>,
        column: Option<usize>,
    ) -> Self {
        Self {
            filename: filename.into(),
            line,
            column,
            end_line: None,
            end_column: None,
        }
    }

    pub fn with_span(
        filename: impl Into<String>,
        line: Option<usize>,
        column: Option<usize>,
        end_line: Option<usize>,
        end_column: Option<usize>,
    ) -> Self {
        Self {
            filename: filename.into(),
            line,
            column,
            end_line,
            end_column,
        }
    }

    /// Create a SourceLocation from an AST node that implements PositionInfo
    pub fn from_node(filename: impl Into<String>, node: &dyn PositionInfo) -> Self {
        let (line, column, end_line, end_column) = node.position_info();
        Self {
            filename: filename.into(),
            line,
            column,
            end_line,
            end_column,
        }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.line, self.column) {
            (Some(line), Some(col)) => {
                if let (Some(end_line), Some(end_col)) = (self.end_line, self.end_column) {
                    if line == end_line {
                        write!(f, "{}:{}:{}-{}", self.filename, line, col, end_col)
                    } else {
                        write!(f, "{}:{}:{}-{}:{}", self.filename, line, col, end_line, end_col)
                    }
                } else {
                    write!(f, "{}:{}:{}", self.filename, line, col)
                }
            }
            (Some(line), None) => write!(f, "{}:{}", self.filename, line),
            _ => write!(f, "{}", self.filename),
        }
    }
}

#[derive(E, Debug)]
pub enum Error {
    #[error("Parsing error at {location}: {message}\nHelp: {help}")]
    ParseError {
        location: SourceLocation,
        message: String,
        help: String,
    },

    #[error("Code generation error at {location}: {message}\nHelp: {help}")]
    CodeGenError {
        location: SourceLocation,
        message: String,
        help: String,
    },

    #[error("Unsupported feature at {location}: {feature} is not yet implemented\nHelp: {help}")]
    UnsupportedFeature {
        location: SourceLocation,
        feature: String,
        help: String,
    },

    #[error("Type error at {location}: {message}\nExpected: {expected}\nFound: {found}\nHelp: {help}")]
    TypeError {
        location: SourceLocation,
        message: String,
        expected: String,
        found: String,
        help: String,
    },

    #[error("Invalid syntax at {location}: {message}\nHelp: {help}")]
    SyntaxError {
        location: SourceLocation,
        message: String,
        help: String,
    },

    // Legacy error types for backward compatibility
    #[error("BinOp type not yet implemented: {:?}", .0)]
    BinOpNotYetImplemented(BinOp),

    #[error("BoolOp type not yet implemented: {:?}", .0)]
    BoolOpNotYetImplemented(BoolOp),

    #[error("Compare type not yet implemented: {:?}", .0)]
    CompareNotYetImplemented(Compare),

    #[error("Expr type not yet implemented: {:?}", .0)]
    ExprNotYetImplemented(Expr),
    #[error("ExprType type not yet implemented: {:?}", .0)]
    ExprTypeNotYetImplemented(ExprType),

    #[error("Unknown type {0}")]
    UnknownType(String),

    #[error("PyO3 Error: {0}")]
    #[from(PyErr)]
    Pyo3Error(PyErr),

    #[error("Statement type not yet implemented: {:?}", .0)]
    StatementNotYetImplemented(StatementType),
    #[error("UnaryOp type not yet implemented: {:?}", .0)]
    UnaryOpNotYetImplemented(UnaryOp),

    #[error("Unknown Error: {0}")]
    #[from(Box<dyn std::error::Error>)]
    UnknownError(Box<dyn std::error::Error>),
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::ParseError { message, .. } 
            | Error::SyntaxError { message, .. } => {
                pyo3::exceptions::PySyntaxError::new_err(message)
            },
            Error::TypeError { message, .. } => {
                pyo3::exceptions::PyTypeError::new_err(message)
            },
            Error::UnsupportedFeature { feature, .. } => {
                pyo3::exceptions::PyNotImplementedError::new_err(format!("Unsupported feature: {}", feature))
            },
            Error::CodeGenError { message, .. } => {
                pyo3::exceptions::PyRuntimeError::new_err(message)
            },
            Error::Pyo3Error(py_err) => py_err,
            _ => pyo3::exceptions::PyRuntimeError::new_err(format!("{}", err)),
        }
    }
}

impl Error {
    /// Create a parsing error with location and helpful guidance
    pub fn parsing_error(
        location: SourceLocation,
        message: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Error::ParseError {
            location,
            message: message.into(),
            help: help.into(),
        }
    }

    /// Create a code generation error with location and helpful guidance
    pub fn codegen_error(
        location: SourceLocation,
        message: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Error::CodeGenError {
            location,
            message: message.into(),
            help: help.into(),
        }
    }

    /// Create an unsupported feature error with location and helpful guidance
    pub fn unsupported_feature(
        location: SourceLocation,
        feature: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Error::UnsupportedFeature {
            location,
            feature: feature.into(),
            help: help.into(),
        }
    }

    /// Create a type error with location and helpful guidance
    pub fn type_error(
        location: SourceLocation,
        message: impl Into<String>,
        expected: impl Into<String>,
        found: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Error::TypeError {
            location,
            message: message.into(),
            expected: expected.into(),
            found: found.into(),
            help: help.into(),
        }
    }

    /// Create a syntax error with location and helpful guidance
    pub fn syntax_error(
        location: SourceLocation,
        message: impl Into<String>,
        help: impl Into<String>,
    ) -> Self {
        Error::SyntaxError {
            location,
            message: message.into(),
            help: help.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_unknown_type() {
        let error = Error::UnknownType("SomeUnknownType".to_string());
        let display = format!("{}", error);
        assert_eq!(display, "Unknown type SomeUnknownType");
    }

    #[test]
    fn test_error_display_unknown_error() {
        let custom_error: Box<dyn std::error::Error> = Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Test error"
        ));
        let error = Error::UnknownError(custom_error);
        let display = format!("{}", error);
        assert!(display.contains("Unknown Error"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_error_debug() {
        let error = Error::UnknownType("TestType".to_string());
        let debug = format!("{:?}", error);
        assert!(debug.contains("UnknownType"));
        assert!(debug.contains("TestType"));
    }

    #[test]
    fn test_result_type_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_err() {
        let result: Result<i32> = Err(Error::UnknownType("TestError".to_string()));
        assert!(result.is_err());
        
        match result {
            Err(Error::UnknownType(msg)) => assert_eq!(msg, "TestError"),
            _ => panic!("Expected UnknownType error"),
        }
    }

    #[test]
    fn test_error_chaining() {
        let result: Result<i32> = Err(Error::UnknownType("ChainTest".to_string()));
        
        let chained_result = result.map_err(|e| Error::UnknownError(Box::new(e)));
        
        assert!(chained_result.is_err());
        match chained_result {
            Err(Error::UnknownError(_)) => (),
            _ => panic!("Expected chained error"),
        }
    }
}
