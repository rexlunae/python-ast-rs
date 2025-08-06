use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    dump, CodeGen, CodeGenContext, Error, ExprType, Node, PythonOptions, SymbolTableScopes,
    PythonOperator, BinaryOperation, FromPythonString, PyAttributeExtractor,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum BinOps {
    Add,
    Sub,
    Mult,
    Div,
    FloorDiv,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    MatMult,

    Unknown,
}

impl FromPythonString for BinOps {
    fn from_python_string(s: &str) -> Option<Self> {
        match s {
            "Add" => Some(BinOps::Add),
            "Sub" => Some(BinOps::Sub),
            "Mult" => Some(BinOps::Mult),
            "Div" => Some(BinOps::Div),
            "FloorDiv" => Some(BinOps::FloorDiv),
            "Mod" => Some(BinOps::Mod),
            "Pow" => Some(BinOps::Pow),
            "LShift" => Some(BinOps::LShift),
            "RShift" => Some(BinOps::RShift),
            "BitOr" => Some(BinOps::BitOr),
            "BitXor" => Some(BinOps::BitXor),
            "BitAnd" => Some(BinOps::BitAnd),
            "MatMult" => Some(BinOps::MatMult),
            _ => None,
        }
    }
    
    fn unknown() -> Self {
        BinOps::Unknown
    }
}

impl PythonOperator for BinOps {
    fn to_rust_op(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            BinOps::Add => Ok(quote!(+)),
            BinOps::Sub => Ok(quote!(-)),
            BinOps::Mult => Ok(quote!(*)),
            BinOps::Div => Ok(quote!(as f64 /)),
            BinOps::FloorDiv => Ok(quote!(/)),
            BinOps::Mod => Ok(quote!(%)),
            BinOps::Pow => Ok(quote!(.pow)),
            BinOps::LShift => Ok(quote!(<<)),
            BinOps::RShift => Ok(quote!(>>)),
            BinOps::BitOr => Ok(quote!(|)),
            BinOps::BitXor => Ok(quote!(^)),
            BinOps::BitAnd => Ok(quote!(&)),
            _ => Err(Error::BinOpNotYetImplemented(BinOp { 
                op: self.clone(), 
                left: Box::new(ExprType::Name(crate::Name { id: "unknown".to_string() })),
                right: Box::new(ExprType::Name(crate::Name { id: "unknown".to_string() })),
            }).into()),
        }
    }
    
    fn precedence(&self) -> u8 {
        match self {
            BinOps::Pow => 8,
            BinOps::Mult | BinOps::Div | BinOps::FloorDiv | BinOps::Mod => 7,
            BinOps::Add | BinOps::Sub => 6,
            BinOps::LShift | BinOps::RShift => 5,
            BinOps::BitAnd => 4,
            BinOps::BitXor => 3,
            BinOps::BitOr => 2,
            _ => 1,
        }
    }
    
    fn is_unknown(&self) -> bool {
        matches!(self, BinOps::Unknown)
    }
}

impl<'a> FromPyObject<'a> for BinOps {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let err_msg = format!("Unimplemented binary op {}", dump(ob, None)?);
        Err(pyo3::exceptions::PyValueError::new_err(
            ob.error_message("<unknown>", err_msg),
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BinOp {
    op: BinOps,
    left: Box<ExprType>,
    right: Box<ExprType>,
}

impl BinaryOperation for BinOp {
    type OperatorType = BinOps;
    
    fn operator(&self) -> &Self::OperatorType {
        &self.op
    }
    
    fn left(&self) -> &ExprType {
        &self.left
    }
    
    fn right(&self) -> &ExprType {
        &self.right
    }
}

impl<'a> FromPyObject<'a> for BinOp {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        log::debug!("ob: {}", dump(ob, None)?);
        
        let op = ob.extract_attr_with_context("op", "binary operator")?;
        let op_type_str = op.extract_type_name("binary operator")?;
        
        let left = ob.extract_attr_with_context("left", "binary operand")?;
        let right = ob.extract_attr_with_context("right", "binary operand")?;
        
        log::debug!("left: {}, right: {}", dump(&left, None)?, dump(&right, None)?);

        let op = BinOps::parse_or_unknown(&op_type_str);
        if matches!(op, BinOps::Unknown) {
            log::debug!("Found unknown BinOp {:?}", op_type_str);
        }

        let left = left.extract().expect("getting binary operator operand");
        let right = right.extract().expect("getting binary operator operand");

        Ok(BinOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

impl CodeGen for BinOp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        // Special handling for Pow operator which needs different syntax
        if matches!(self.op, BinOps::Pow) {
            let left = self.left.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let right = self.right.clone().to_rust(ctx, options, symbols)?;
            return Ok(quote!((#left).pow(#right)));
        }
        
        // For Div, we need to cast to f64
        if matches!(self.op, BinOps::Div) {
            let left = self.left.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let right = self.right.clone().to_rust(ctx, options, symbols)?;
            return Ok(quote!((#left) as f64 / (#right) as f64));
        }
        
        // Special handling for list addition (concatenation)
        if matches!(self.op, BinOps::Add) {
            let left = self.left.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let right = self.right.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let left_str = left.to_string();
            let right_str = right.to_string();
            
            // Check if we're adding vectors or lists together
            if left_str.contains("vec !") || right_str.contains("iter ()") || right_str.contains("sys :: argv") {
                // This is vector concatenation - use Vec::extend pattern
                return Ok(quote! {
                    {
                        let mut vec = #left;
                        vec.extend(#right);
                        vec
                    }
                });
            }
        }
        
        // Use the generic binary operation implementation for everything else
        self.generate_rust_code(ctx, options, symbols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_parse_test;

    create_parse_test!(test_add, "1 + 2", "test_case.py");
    create_parse_test!(test_subtract, "1 - 2", "test_case.py");
    create_parse_test!(test_multiply, "3 * 4", "test_case.py");
    create_parse_test!(test_divide, "8 / 2", "test_case.py");
    create_parse_test!(test_power, "2 ** 3", "test_case.py");
    create_parse_test!(test_modulo, "10 % 3", "test_case.py");
    
    #[test]
    fn test_operator_precedence() {
        let add_op = BinOps::Add;
        let mul_op = BinOps::Mult;
        let pow_op = BinOps::Pow;
        
        assert!(pow_op.precedence() > mul_op.precedence());
        assert!(mul_op.precedence() > add_op.precedence());
    }
    
    #[test]
    fn test_unknown_operator() {
        let unknown_op = BinOps::Unknown;
        assert!(unknown_op.is_unknown());
        assert!(unknown_op.to_rust_op().is_err());
    }
    
    #[test]
    fn test_from_python_string() {
        assert_eq!(BinOps::from_python_string("Add"), Some(BinOps::Add));
        assert_eq!(BinOps::from_python_string("Unknown"), None);
        assert_eq!(BinOps::parse_or_unknown("Invalid"), BinOps::Unknown);
    }
}
