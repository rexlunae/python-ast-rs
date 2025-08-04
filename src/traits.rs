use proc_macro2::TokenStream;
use quote::quote;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyTypeMethods};
use crate::{CodeGen, CodeGenContext, PythonOptions, SymbolTableScopes, ExprType};

/// Common trait for Python operators that can be converted to Rust tokens.
pub trait PythonOperator: Clone + std::fmt::Debug {
    /// Convert the operator to its Rust equivalent TokenStream.
    fn to_rust_op(&self) -> Result<TokenStream, Box<dyn std::error::Error>>;
    
    /// Get the operator precedence for proper parenthesization.
    fn precedence(&self) -> u8 {
        0 // Default precedence
    }
    
    /// Check if this operator is unknown/unimplemented.
    fn is_unknown(&self) -> bool;
}

/// Common trait for binary operations (binary ops, bool ops, comparisons).
pub trait BinaryOperation: Clone + std::fmt::Debug {
    type OperatorType: PythonOperator;
    
    /// Get the operator type.
    fn operator(&self) -> &Self::OperatorType;
    
    /// Get the left operand.
    fn left(&self) -> &ExprType;
    
    /// Get the right operand.
    fn right(&self) -> &ExprType;
    
    /// Generate Rust code for this binary operation.
    fn generate_rust_code(
        &self,
        ctx: CodeGenContext,
        options: PythonOptions,
        symbols: SymbolTableScopes,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let left = self.left()
            .clone()
            .to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let right = self.right()
            .clone()
            .to_rust(ctx, options, symbols)?;
        let op = self.operator().to_rust_op()?;
        
        Ok(quote!((#left) #op (#right)))
    }
}

/// Trait for extracting Python attributes with consistent error handling.
pub trait PyAttributeExtractor {
    /// Extract an attribute with context-aware error messages.
    fn extract_attr_with_context(&self, attr: &str, context: &str) -> pyo3::PyResult<pyo3::Bound<pyo3::PyAny>>;
    
    /// Extract a type name with error handling.
    fn extract_type_name(&self, context: &str) -> pyo3::PyResult<String>;
}

impl<'py> PyAttributeExtractor for pyo3::Bound<'py, pyo3::PyAny> {
    fn extract_attr_with_context(&self, attr: &str, context: &str) -> pyo3::PyResult<pyo3::Bound<pyo3::PyAny>> {
        use crate::Node;
        self.getattr(attr).map_err(|_| {
            pyo3::exceptions::PyAttributeError::new_err(
                self.error_message("<unknown>", format!("error getting {}", context))
            )
        })
    }
    
    fn extract_type_name(&self, context: &str) -> pyo3::PyResult<String> {
        use crate::Node;
        use pyo3::prelude::*;
        
        let type_name = self.get_type().name().map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err(
                self.error_message("<unknown>", format!("extracting type name for {}", context))
            )
        })?;
        
        type_name.extract()
    }
}

/// Trait for consistent handling of position information in AST nodes.
pub trait PositionInfo {
    /// Get all position fields as a tuple (lineno, col_offset, end_lineno, end_col_offset).
    fn position_info(&self) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>);
    
    /// Check if this node has position information.
    fn has_position(&self) -> bool {
        let (lineno, col_offset, _, _) = self.position_info();
        lineno.is_some() && col_offset.is_some()
    }
}

/// Trait for AST nodes that can provide debugging information.
pub trait DebugInfo {
    /// Get a human-readable description of this node.
    fn debug_description(&self) -> String;
    
    /// Get the node type name.
    fn node_type(&self) -> &'static str;
}

/// Trait for operations that can be chained (like comparison operations).
pub trait ChainableOperation {
    type Operand;
    type Operator;
    
    /// Get all operands in the chain.
    fn operands(&self) -> Vec<&Self::Operand>;
    
    /// Get all operators in the chain.
    fn operators(&self) -> Vec<&Self::Operator>;
    
    /// Generate chained Rust code.
    fn generate_chained_rust(
        &self,
        ctx: CodeGenContext,
        options: PythonOptions,
        symbols: SymbolTableScopes,
    ) -> Result<TokenStream, Box<dyn std::error::Error>>;
}

/// Helper trait for converting Python string literals to enum variants.
pub trait FromPythonString: Sized {
    /// Convert a Python operator string to the enum variant.
    fn from_python_string(s: &str) -> Option<Self>;
    
    /// Get the unknown/default variant.
    fn unknown() -> Self;
    
    /// Parse from string with fallback to unknown.
    fn parse_or_unknown(s: &str) -> Self {
        Self::from_python_string(s).unwrap_or_else(Self::unknown)
    }
}

/// Trait for generating consistent error messages across the codebase.
pub trait ErrorContext {
    /// Generate a standardized error message with context.
    fn with_context(&self, operation: &str) -> String;
}

impl<T: std::fmt::Debug> ErrorContext for T {
    fn with_context(&self, operation: &str) -> String {
        format!("Error during {}: {:?}", operation, self)
    }
}