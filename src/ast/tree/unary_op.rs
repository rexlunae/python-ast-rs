use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::quote;

use crate::{
    dump, CodeGen, CodeGenContext, Error, ExprType, Node, PythonOptions, SymbolTableScopes,
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
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
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
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let py = ob.py();

        log::debug!("ob: {}", dump(ob, None)?);
        let op = ob.as_unbound().getattr(py, "op").expect(
            ob.error_message("<unknown>", "error getting unary operator")
                .as_str(),
        );

        let bound_op = op.bind(py);
        let op_type = bound_op.get_type().name().expect(
            ob.error_message(
                "<unknown>",
                format!("extracting type name {:?} for unary operator", op),
            )
            .as_str(),
        );

        let operand = ob.as_unbound().getattr(py, "operand").expect(
            ob.error_message("<unknown>", "error getting unary operand")
                .as_str(),
        );

        let op = match op_type.extract::<String>()?.as_str() {
            "Invert" => Ops::Invert,
            "Not" => Ops::Not,
            "UAdd" => Ops::UAdd,
            "USub" => Ops::USub,
            _ => {
                log::debug!("{:?}", op);
                Ops::Unknown
            }
        };

        log::debug!("operand: {}", dump(&operand.bind(py), None)?);
        let bound_op = operand.bind(py);
        let operand = ExprType::extract_bound(bound_op).expect("getting unary operator operand");

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
            _ => Err(Error::UnaryOpNotYetImplemented(self).into())
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
