use std::borrow::Borrow;

use thiserror::Error as E;
use pyo3::PyErr;

use crate::{
    BinOp, BoolOp, Compare, Expr, ExprType, StatementType, UnaryOp,
};

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
    UnknownError(Box<dyn std::error::Error>)
}

pub type Result<T> = std::result::Result<T, Error>;
