use pyo3::{FromPyObject, PyAny, PyResult};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use log::debug;

use crate::tree::{Call, Constant};
use crate::codegen::{CodeGen, CodeGenError, PythonContext};

#[derive(Clone, Debug, FromPyObject)]
pub enum ExprType {
    /*BoolOp(),
    NamedExpr(),
    BinOp(),
    UnaryOp(),
    Lambda(),
    IfExp(),
    Dict(),
    Set(),
    ListComp(),
    SetComp(),
    DictComp(),
    GeneratorExp(),
    Await(),
    Yield(),
    YieldFrom(),
    Compare(),*/
    Call(Call),
    /*FormattedValue(),
    JoinedStr(),*/
    Constant(Constant),
    /*Attribute(),
    Subscript(),
    Starred(),
    Name(),
    List(),
    Tuple(),
    Slice(),*/

    Unimplemented(String),
}

/// An Expr only contains a single value key, which leads to the actual expression,
/// which is one of several types.
#[derive(Clone, Debug)]
pub struct Expr {
    pub value: ExprType,
}

impl<'a> FromPyObject<'a> for Expr {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let ob_value = ob.getattr("value")?;
        let expr_type = ob_value.get_type().name()?;
        debug!("expr ob_type: {}...{}", expr_type, crate::ast_dump(ob_value, Some(4))?);
        let r = match expr_type {
            "Call" => {
                let et = Call::extract(ob_value)?;
                Ok(Self{value: ExprType::Call(et)})
            },
            "Constant" => Ok(Self{value: ExprType::Constant(Constant::extract(ob)?)}),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unimplemented expression type {}, {}", expr_type, crate::ast_dump(ob, None)?)))
        };
        debug!("ret: {:?}", r);
        r
    }
}

impl<'a> CodeGen for Expr {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self.value {
            ExprType::Call(call) => {
                let name = format_ident!("{}", call.func.id);
                let mut arg_stream = proc_macro2::TokenStream::new();

                for s in call.args.iter() {
                    arg_stream.extend(s.clone().to_rust(ctx)?);
                }
                Ok(quote!{#name(#arg_stream)})
            },
            ExprType::Constant(constant) => {
                constant.to_rust(ctx)
            },
                //Expr::Break => Ok(quote!{break;}),
            _ => {
                let error = CodeGenError(format!("Expr not implemented {:?}", self), None);
                Err(Box::new(error))
            }
        }
    }

    // override the default to allow functions to be compiled as trait members.
    fn to_rust_trait_member(&self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        (*self).clone().to_rust(ctx)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Name;


    #[test]
    fn check_call_expression() {
        let expression = Expr{
                    value: ExprType::Call(Call{
                        func: Name{id: "test".to_string()},
                        args: Vec::new(),
                        keywords: Vec::new(),
                    })
        };
        let mut ctx = PythonContext::default();
        let tokens = expression.clone().to_rust(&mut ctx).unwrap();
        assert_eq!(tokens.to_string(), quote!(test()).to_string());
    }

}
