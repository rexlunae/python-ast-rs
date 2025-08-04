
use pyo3::PyErr;
use thiserror::Error as E;

use crate::{BinOp, BoolOp, Compare, Expr, ExprType, StatementType, UnaryOp};

#[derive(E, Debug)]
pub enum Error {
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
