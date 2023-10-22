use pyo3::{FromPyObject, PyAny, PyResult};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use log::debug;

use crate::tree::{Call, Constant};
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, CodeGenContext};

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
        let ob_value = ob.getattr("value").expect(format!("extracting object value {:?} in expression", ob).as_str());
        let expr_type = ob_value.get_type().name().expect(format!("extracting type name {:?} in expression", ob_value).as_str());
        debug!("[0] expr ob_type: {}...{}", expr_type, crate::ast_dump(ob_value, Some(4))?);
        let r = match expr_type {
            "Call" => {
                let et = Call::extract(ob_value).expect(format!("parsing Call expression {:?}", ob_value).as_str());
                Ok(Self{value: ExprType::Call(et)})
            },
            "Constant" => {
                debug!("[1] expression ob: {}", crate::ast_dump(ob_value, Some(4))?);
                let c = Constant::extract(ob_value)
                    .expect(format!("extracting Constant in expression {:?}", crate::ast_dump(ob_value, Some(4))?).as_str());
                Ok(Self {
                    value: ExprType::Constant(c)
                })
            },
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unimplemented expression type {}, {}", expr_type, crate::ast_dump(ob, None)?)))
        };
        debug!("ret: {:?}", r);
        r
    }
}

impl<'a> CodeGen for Expr {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self.value {
            ExprType::Call(call) => {
                let name = format_ident!("{}", call.func.id);
                let mut arg_stream = proc_macro2::TokenStream::new();

                for s in call.args {
                    arg_stream.extend(s.clone().to_rust(ctx, options.clone()).expect(format!("parsing argument {:?}", s).as_str()));
                }
                Ok(quote!{#name(#arg_stream)})
            },
            ExprType::Constant(constant) => {
                constant.to_rust(ctx, options)
            },
                //Expr::Break => Ok(quote!{break;}),
            _ => {
                let error = CodeGenError(format!("Expr not implemented {:?}", self), None);
                Err(Box::new(error))
            }
        }
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
        let options = PythonOptions::default();
        let tokens = expression.clone().to_rust(CodeGenContext::Module, options).unwrap();
        assert_eq!(tokens.to_string(), quote!(test()).to_string());
    }

}
