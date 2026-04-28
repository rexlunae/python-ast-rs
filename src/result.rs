//! Error and result types for the `python-ast` crate.
//!
//! This module re-exports [`anyhow_tracing::Error`] / [`anyhow_tracing::Result`] as the
//! crate-wide [`Error`] / [`Result`] types so that errors carry **structured fields**
//! (location, feature, expected, found, …) instead of pre-formatted `String`s, which
//! allows tracing/logging consumers — and downstream code that wants to recover the
//! typed payload via [`anyhow_tracing::Error::downcast_ref`] — to inspect the original
//! data without having to re-parse a stringified message.
//!
//! The concrete error *kinds* (e.g. [`ParseError`], [`UnsupportedFeature`],
//! [`BinOpNotYetImplemented`]) are exposed as small `thiserror`-derived structs that
//! implement [`std::error::Error`]. They are wrapped into [`anyhow_tracing::Error`] at
//! the construction site (see the helper constructors on this module's `Error`-shim
//! re-exports below: [`parsing_error`], [`codegen_error`], [`unsupported_feature`],
//! [`type_error`], [`syntax_error`]). The wrapping carries named fields so structured
//! tracing subscribers see fields rather than a stringified blob, while downcasts
//! still recover the typed payload.

use pyo3::PyErr;
use thiserror::Error as ThisError;

use crate::{BinOp, BoolOp, Compare, Expr, ExprType, PositionInfo, StatementType, UnaryOp};

// Re-export the anyhow-tracing types and macros so the rest of the crate has a
// single import path: `crate::{Error, Result, Context, anyhow, bail, ensure}`.
pub use anyhow_tracing::{Context, Error, Result};
// Macros are exported at the root by `#[macro_export]`; re-export them at this
// path for ergonomic use as `crate::result::{anyhow, bail, ensure}` if desired.
pub use anyhow_tracing::{anyhow, bail, ensure};

/// Location information for error reporting.
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

    /// Create a SourceLocation from an AST node that implements PositionInfo.
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
                        write!(
                            f,
                            "{}:{}:{}-{}:{}",
                            self.filename, line, col, end_line, end_col
                        )
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

// ---------------------------------------------------------------------------
// Concrete (downcast-able) error kinds.
//
// These hold typed payloads so callers can recover them via
// `err.downcast_ref::<ParseError>()` etc. The original String-typed `message`,
// `help`, `expected`, `found` payloads are *removed* from the typed payload —
// they are now carried as named fields on the [`anyhow_tracing::Error`] wrapper
// (see the constructor helpers below). This is the core of the migration: no
// premature String-ification of structured data.
// ---------------------------------------------------------------------------

/// Python parsing failed.
#[derive(Debug, ThisError)]
#[error("parsing error at {location}")]
pub struct ParseError {
    pub location: SourceLocation,
}

/// Code generation failed.
#[derive(Debug, ThisError)]
#[error("code generation error at {location}")]
pub struct CodeGenError {
    pub location: SourceLocation,
}

/// A Python feature is not yet implemented in the transpiler.
#[derive(Debug, ThisError)]
#[error("unsupported feature `{feature}` at {location}")]
pub struct UnsupportedFeature {
    pub location: SourceLocation,
    pub feature: String,
}

/// A type mismatch was detected during code generation.
#[derive(Debug, ThisError)]
#[error("type error at {location}")]
pub struct TypeError {
    pub location: SourceLocation,
}

/// The Python source contained invalid syntax.
#[derive(Debug, ThisError)]
#[error("syntax error at {location}")]
pub struct SyntaxError {
    pub location: SourceLocation,
}

/// A binary operator the transpiler does not yet handle. Carries the original
/// `BinOp` AST node so callers can inspect it without having to re-parse a
/// stringified message.
#[derive(Debug, ThisError)]
#[error("BinOp type not yet implemented: {0:?}")]
pub struct BinOpNotYetImplemented(pub BinOp);

#[derive(Debug, ThisError)]
#[error("BoolOp type not yet implemented: {0:?}")]
pub struct BoolOpNotYetImplemented(pub BoolOp);

#[derive(Debug, ThisError)]
#[error("Compare type not yet implemented: {0:?}")]
pub struct CompareNotYetImplemented(pub Compare);

#[derive(Debug, ThisError)]
#[error("Expr type not yet implemented: {0:?}")]
pub struct ExprNotYetImplemented(pub Expr);

#[derive(Debug, ThisError)]
#[error("ExprType type not yet implemented: {0:?}")]
pub struct ExprTypeNotYetImplemented(pub ExprType);

#[derive(Debug, ThisError)]
#[error("Statement type not yet implemented: {0:?}")]
pub struct StatementNotYetImplemented(pub StatementType);

#[derive(Debug, ThisError)]
#[error("UnaryOp type not yet implemented: {0:?}")]
pub struct UnaryOpNotYetImplemented(pub UnaryOp);

#[derive(Debug, ThisError)]
#[error("Unknown type {0}")]
pub struct UnknownType(pub String);

// ---------------------------------------------------------------------------
// Construction helpers that build a structured `anyhow_tracing::Error`.
//
// Each helper:
//   1. Builds the typed payload struct so it can be recovered by `downcast_ref`,
//   2. Wraps it into an `anyhow_tracing::Error`,
//   3. Attaches the previously-stringified data as **named fields** (location,
//      help, expected, found, …) so structured logging sees them as fields.
// ---------------------------------------------------------------------------

/// Wrap a typed error struct into a structured [`Error`], preserving the
/// typed payload as the source so callers can recover it via `downcast_ref`.
fn into_error<E>(typed: E) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    Error::from(anyhow::Error::from(typed))
}

/// Create a parsing error with location and helpful guidance.
pub fn parsing_error(
    location: SourceLocation,
    message: impl std::fmt::Display,
    help: impl std::fmt::Display,
) -> Error {
    let location_str = location.to_string();
    into_error(ParseError { location })
        .with_field("location", location_str)
        .with_field("message", message)
        .with_field("help", help)
}

/// Create a code generation error with location and helpful guidance.
pub fn codegen_error(
    location: SourceLocation,
    message: impl std::fmt::Display,
    help: impl std::fmt::Display,
) -> Error {
    let location_str = location.to_string();
    into_error(CodeGenError { location })
        .with_field("location", location_str)
        .with_field("message", message)
        .with_field("help", help)
}

/// Create an unsupported-feature error with location and helpful guidance.
pub fn unsupported_feature(
    location: SourceLocation,
    feature: impl Into<String>,
    help: impl std::fmt::Display,
) -> Error {
    let feature = feature.into();
    let location_str = location.to_string();
    into_error(UnsupportedFeature {
        location,
        feature: feature.clone(),
    })
    .with_field("location", location_str)
    .with_field("feature", feature)
    .with_field("help", help)
}

/// Create a type error with location and helpful guidance.
pub fn type_error(
    location: SourceLocation,
    message: impl std::fmt::Display,
    expected: impl std::fmt::Display,
    found: impl std::fmt::Display,
    help: impl std::fmt::Display,
) -> Error {
    let location_str = location.to_string();
    into_error(TypeError { location })
        .with_field("location", location_str)
        .with_field("message", message)
        .with_field("expected", expected)
        .with_field("found", found)
        .with_field("help", help)
}

/// Create a syntax error with location and helpful guidance.
pub fn syntax_error(
    location: SourceLocation,
    message: impl std::fmt::Display,
    help: impl std::fmt::Display,
) -> Error {
    let location_str = location.to_string();
    into_error(SyntaxError { location })
        .with_field("location", location_str)
        .with_field("message", message)
        .with_field("help", help)
}

// ---------------------------------------------------------------------------
// Boundary adapters.
//
// `anyhow_tracing::Error` does not have a blanket `From<E: std::error::Error>`
// impl, so we provide a small extension trait that converts any
// `Result<T, E: std::error::Error>` into our crate-wide [`Result<T>`]. This
// keeps `?` ergonomic at the boundary where foreign errors (e.g. `PyErr`,
// `syn::Error`, `std::io::Error`) flow into a function returning [`Result`].
//
// Use `.into_err()` instead of `?` directly when the source error doesn't have
// a `From` impl into [`Error`]:
//
//     let dumped = dump(ob, None).into_err()?; // PyErr -> Error
// ---------------------------------------------------------------------------

/// Converts foreign error results into the crate's structured [`Result`] type.
pub trait IntoErr<T> {
    /// Convert the error into [`Error`], preserving the original error as the
    /// source so callers can still downcast to the concrete type.
    fn into_err(self) -> Result<T>;
}

impl<T, E> IntoErr<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn into_err(self) -> Result<T> {
        self.map_err(|e| Error::from(anyhow::Error::from(e)))
    }
}

/// Convert any `std::error::Error` into the crate-wide [`Error`].
pub fn err_from<E>(e: E) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    Error::from(anyhow::Error::from(e))
}

// ---------------------------------------------------------------------------
// PyO3 boundary conversion.
//
// We can't `impl From<Error> for PyErr` because of orphan rules (both types
// are foreign). Instead, expose a free function `error_to_pyerr` that callers
// invoke explicitly at the FFI boundary, and which performs structural
// downcasting to pick an appropriate Python exception class. The named fields
// on the error are appended to the message so the Python caller still sees
// the structured context (this is the only place we stringify).
// ---------------------------------------------------------------------------

/// Convert a crate [`Error`] into a [`PyErr`] suitable for returning across
/// the PyO3 boundary. Performs structural downcasting so e.g.
/// [`SyntaxError`] becomes `PySyntaxError` and [`UnsupportedFeature`] becomes
/// `PyNotImplementedError`.
pub fn error_to_pyerr(err: Error) -> PyErr {
    use pyo3::exceptions::*;

    let display = format!("{:#}", err);

    if err.is::<ParseError>() || err.is::<SyntaxError>() {
        PySyntaxError::new_err(display)
    } else if err.is::<TypeError>() {
        PyTypeError::new_err(display)
    } else if err.is::<UnsupportedFeature>() {
        PyNotImplementedError::new_err(display)
    } else if err.is::<CodeGenError>() {
        PyRuntimeError::new_err(display)
    } else {
        PyRuntimeError::new_err(display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_type_downcast() {
        let err: Error = err_from(UnknownType("SomeUnknownType".to_string()));
        let display = format!("{}", err);
        assert!(display.contains("SomeUnknownType"));
        let inner = err.downcast_ref::<UnknownType>().expect("should downcast");
        assert_eq!(inner.0, "SomeUnknownType");
    }

    #[test]
    fn test_parsing_error_carries_named_fields() {
        let loc = SourceLocation::new("foo.py");
        let err = parsing_error(loc.clone(), "boom", "fix it");

        // Typed payload is downcast-able.
        let typed = err.downcast_ref::<ParseError>().expect("downcast");
        assert_eq!(typed.location, loc);

        // Named fields are preserved (no premature String-ification of the
        // logical fields — they're stored as separate keys).
        assert_eq!(err.get_field("message"), Some("boom"));
        assert_eq!(err.get_field("help"), Some("fix it"));
        assert!(err.get_field("location").is_some());
    }

    #[test]
    fn test_unsupported_feature_named_fields() {
        let loc = SourceLocation::new("bar.py");
        let err = unsupported_feature(loc, "match-statement", "use if/elif chains");
        assert_eq!(err.get_field("feature"), Some("match-statement"));
        assert_eq!(err.get_field("help"), Some("use if/elif chains"));
        let typed = err.downcast_ref::<UnsupportedFeature>().expect("downcast");
        assert_eq!(typed.feature, "match-statement");
    }

    #[test]
    fn test_type_error_carries_expected_and_found() {
        let loc = SourceLocation::new("baz.py");
        let err = type_error(loc, "mismatch", "int", "str", "convert it");
        assert_eq!(err.get_field("expected"), Some("int"));
        assert_eq!(err.get_field("found"), Some("str"));
        assert!(err.downcast_ref::<TypeError>().is_some());
    }

    #[test]
    fn test_into_err_adapter() {
        let pyresult: std::result::Result<(), std::io::Error> =
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "missing"));
        let r: Result<()> = pyresult.into_err();
        let e = r.expect_err("must be err");
        let display = format!("{}", e);
        assert!(display.contains("missing"));
    }

    #[test]
    fn test_result_ok_passthrough() {
        let r: Result<i32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }
}
