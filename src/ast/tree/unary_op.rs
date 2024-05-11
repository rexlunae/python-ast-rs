use proc_macro2::TokenStream;
use pyo3::{FromPyObject, PyAny, PyResult};
use quote::quote;

use crate::{
    dump, CodeGen, CodeGenContext, CodeGenError, ExprType, Node, PythonOptions, SymbolTableScopes,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Ops {
    Invert,
    Not,
    UAdd,
    USub,

    Unknown,
}

impl<'a> FromPyObject<'a> for Ops {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let err_msg = format!("Unimplemented unary op {}", dump(ob, None)?);
        Err(pyo3::exceptions::PyValueError::new_err(
            ob.error_message("<unknown>", err_msg),
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UnaryOp {
    op: Ops,
    operand: Box<ExprType>,
}

impl<'a> FromPyObject<'a> for UnaryOp {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        log::debug!("ob: {}", dump(ob, None)?);
        let op = ob.getattr("op").expect(
            ob.error_message("<unknown>", "error getting unary operator")
                .as_str(),
        );

        let op_type = op.get_type().name().expect(
            ob.error_message(
                "<unknown>",
                format!("extracting type name {:?} for unary operator", op),
            )
            .as_str(),
        );

        let operand = ob.getattr("operand").expect(
            ob.error_message("<unknown>", "error getting unary operand")
                .as_str(),
        );

        let op = match op_type {
            "Invert" => Ops::Invert,
            "Not" => Ops::Not,
            "UAdd" => Ops::UAdd,
            "USub" => Ops::USub,
            _ => {
                log::debug!("{:?}", op);
                Ops::Unknown
            }
        };

        log::debug!("operand: {}", dump(operand, None)?);
        let operand = ExprType::extract(operand).expect("getting unary operator operand");

        return Ok(UnaryOp {
            op: op,
            operand: Box::new(operand),
        });
    }
}

impl CodeGen for UnaryOp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let operand = self.operand.clone().to_rust(ctx, options, symbols)?;
        match self.op {
            Ops::Invert | Ops::Not => Ok(quote!(!#operand)),
            Ops::UAdd => Ok(quote!(+#operand)),
            Ops::USub => Ok(quote!(-#operand)),
            _ => {
                let error =
                    CodeGenError::NotYetImplemented(format!("UnaryOp not implemented {:?}", self));
                Err(error.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        let options = PythonOptions::default();
        let result = crate::parse("not True", "test").unwrap();
        log::info!("Python tree: {:?}", result);
        //log::info!("{}", result);

        let code = result
            .to_rust(
                CodeGenContext::Module("test".to_string()),
                options,
                SymbolTableScopes::new(),
            )
            .unwrap();
        log::info!("module: {:?}", code);
    }
}
