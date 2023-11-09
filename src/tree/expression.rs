use pyo3::{FromPyObject, PyAny, PyResult};
use crate::codegen::Node;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};


use crate::tree::{Call, Constant, UnaryOp, Name};
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, CodeGenContext};

#[derive(Clone, Debug, FromPyObject)]
pub enum ExprType {
    /*BoolOp(),
    NamedExpr(),
    BinOp(),*/
    UnaryOp(UnaryOp),
    /*Lambda(),
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
    Starred(),*/
    Name(Name),
    /*List(),
    Tuple(),
    Slice(),*/
    NoneType(Constant),

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
        let err_msg = format!("extracting object value {:?} in expression", ob);

        let ob_value = ob.getattr("value").expect(
            ob.error_message("<unknown>", err_msg.as_str()).as_str()
        );
        let expr_type = ob_value.get_type().name().expect(
            ob.error_message("<unknown>", format!("extracting type name {:?} in expression", ob_value).as_str()).as_str()
        );
        let r = match expr_type {
            "Name" => {
                let name = Name::extract(ob_value).expect(
                    ob.error_message("<unknown>", format!("parsing Call expression {:?}", ob_value).as_str()).as_str()
                );
                Ok(Self{value: ExprType::Name(name)})
            }
            "Call" => {
                let et = Call::extract(ob_value).expect(
                    ob.error_message("<unknown>", format!("parsing Call expression {:?}", ob_value).as_str()).as_str()
                );
                Ok(Self{value: ExprType::Call(et)})
            },
            "Constant" => {
                let c = Constant::extract(ob_value)
                    .expect(
                        ob.error_message("<unknown>",
                            format!("extracting Constant in expression {:?}", crate::ast_dump(ob_value, None)?
                        ).as_str()).as_str()
                    );
                Ok(Self {
                    value: ExprType::Constant(c)
                })
            },
            "UnaryOp" => {
                let c = UnaryOp::extract(ob_value)
                    .expect(
                        ob.error_message("<unknown>",
                            format!("extracting UnaryOp in expression {:?}", crate::ast_dump(ob_value, None)?
                        ).as_str()).as_str()
                    );
                Ok(Self {
                    value: ExprType::UnaryOp(c)
                })

            },
            // In sitations where an expression is optional, we may see a NoneType expressions.
            "NoneType" => Ok(Expr{value: ExprType::NoneType(Constant(None))}),
            _ => {
                let err_msg = format!("Unimplemented expression type {}, {}", expr_type, crate::ast_dump(ob, None)?);
                Err(pyo3::exceptions::PyValueError::new_err(
                    ob.error_message("<unknown>", err_msg.as_str())
                ))
            }
        };
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
            ExprType::UnaryOp(operand) => {
                operand.to_rust(ctx, options)
            },
            // NoneType expressions generate no code.
            ExprType::NoneType(_c) => Ok(quote!()),
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
