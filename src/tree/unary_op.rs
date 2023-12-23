use pyo3::{FromPyObject, PyAny, PyResult};
use crate::codegen::Node;
use proc_macro2::TokenStream;
use quote::{quote};

use crate::tree::{Constant};
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, CodeGenContext};
use crate::symbols::SymbolTableScopes;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Ops {
    USub,
    Unknown,
}

impl<'a> FromPyObject<'a> for Ops {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let err_msg = format!("Unimplemented unary op {}", crate::ast_dump(ob, None)?);
        Err(pyo3::exceptions::PyValueError::new_err(
            ob.error_message("<unknown>", err_msg.as_str())
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnaryOp {
    op: Ops,
    operand: Box<Constant>,
}

impl<'a> FromPyObject<'a> for UnaryOp {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        log::debug!("ob: {}", crate::ast_dump(ob, None)?);
        let op = ob.getattr("op").expect(
            ob.error_message("<unknown>", "error getting unary operator").as_str()
        );

        let op_type = op.get_type().name().expect(
            ob.error_message("<unknown>", format!("extracting type name {:?} for unary operator", op).as_str()).as_str()
        );

        let operand = ob.getattr("operand").expect(
            ob.error_message("<unknown>", "error getting unary operand").as_str()
        );

        let op = match op_type {
            "USub" => Ops::USub,
            _ => {
                log::debug!("{:?}", op);
                Ops::Unknown
            }
        };

        log::debug!("operand: {}", crate::ast_dump(operand, None)?);
        let operand = Constant::extract(operand).expect("getting unary operator operand");

        return Ok(UnaryOp{
            op: op,
            operand: Box::new(operand),
        });

    }
}

impl<'a> CodeGen for UnaryOp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self.op {
            Ops::USub => {
                let e = self.operand.clone().to_rust(ctx, options, symbols)?;
                Ok(quote!(-#e))
            },
            _ => {
                let error = CodeGenError(format!("UnaryOp not implemented {:?}", self), None);
                Err(Box::new(error))
            }
        }
    }
}
