use pyo3::{FromPyObject, PyAny, PyResult};
use crate::codegen::{Node};
//use crate::pytypes::{ListLike};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use serde::{Serialize, Deserialize};

use crate::tree::{Call, Constant, UnaryOp, Name};
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, CodeGenContext};
use crate::symbols::SymbolTableScopes;

/// Mostly this shouldn't be used, but it exists so that we don't have to manually implement FromPyObject on all of ExprType
#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Container<T>(pub T);

impl<'a> FromPyObject<'a> for Container<crate::pytypes::List<ExprType>> {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let list = crate::pytypes::List::<ExprType>::new();

        log::debug!("pylist: {}", crate::ast_dump(ob, Some(4))?);
        let _converted_list: Vec<&PyAny> = ob.extract()?;
        for item in ob.iter().expect("extracting list") {
            log::debug!("item: {:?}", item);
        }

        Ok(Self(list))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExprType {
    /*BoolOp(BoolOp),
    NamedExpr(NamedExpr),
    BinOp(),*/
    UnaryOp(UnaryOp),
    /*Lambda(Lamda),
    IfExp(IfExp),
    Dict(Dict),
    Set(Set),
    ListComp(ListComp),
    SetComp(SetComp),
    DictComp(DictComp),
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
    List(Container<crate::pytypes::List<ExprType>>),
    /*Tuple(),
    Slice(),*/
    NoneType(Constant),

    Unimplemented(String),
}

impl<'a> FromPyObject<'a> for ExprType {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        log::debug!("exprtype ob: {}", crate::ast_dump(ob, Some(4))?);

        let expr_type = ob.get_type().name().expect(
            ob.error_message("<unknown>", format!("extracting type name {:?} in expression", ob).as_str()).as_str()
        );
        log::debug!("expression type: {}, value: {}", expr_type, crate::ast_dump(ob, None)?);

        let r = match expr_type {
            "Name" => {
                let name = Name::extract(ob).expect(
                    ob.error_message("<unknown>", format!("parsing Call expression {:?}", ob).as_str()).as_str()
                );
                Ok(Self::Name(name))
            }
            "Call" => {
                let et = Call::extract(ob).expect(
                    ob.error_message("<unknown>", format!("parsing Call expression {:?}", ob).as_str()).as_str()
                );
                Ok(Self::Call(et))
            },
            "Constant" => {
                log::debug!("constant: {}", crate::ast_dump(ob, None)?);
                let c = Constant::extract(ob)
                    .expect(
                        ob.error_message("<unknown>",
                            format!("extracting Constant in expression {:?}", crate::ast_dump(ob, None)?
                        ).as_str()).as_str()
                    );
                Ok(Self::Constant(c))
            },
            "List" => {
                //let list = crate::pytypes::List::<ExprType>::new();
                let list = Container::extract(ob).expect("extracting List");
                Ok(Self::List(list))
            }
            "UnaryOp" => {
                let c = UnaryOp::extract(ob)
                    .expect(
                        ob.error_message("<unknown>",
                            format!("extracting UnaryOp in expression {:?}", crate::ast_dump(ob, None)?
                        ).as_str()).as_str()
                    );
                Ok(Self::UnaryOp(c))
            },
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

impl<'a> CodeGen for ExprType {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            ExprType::List(l) => {
                let mut ts = TokenStream::new();
                for li in l.0 {
                    let code = li.clone().to_rust(ctx, options.clone(), symbols.clone()).expect(format!("Extracting list item {:?}", li).as_str());
                    ts.extend(code);
                    ts.extend(quote!(,));
                }
                Ok(ts)
            },
            ExprType::NoneType(c) => c.to_rust(ctx, options, symbols),
            _ => {
                let error = CodeGenError(format!("Expr not implemented converting to Rust {:?}", self), None);
                Err(Box::new(error))
            }
        }
    }
}

/// An Expr only contains a single value key, which leads to the actual expression,
/// which is one of several types.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    pub value: ExprType,
    pub ctx: Option<String>,
}

impl<'a> FromPyObject<'a> for Expr {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let err_msg = format!("extracting object value {:?} in expression", ob);

        let ob_value = ob.getattr("value").expect(
            ob.error_message("<unknown>", err_msg.as_str()).as_str()
        );
        log::debug!("ob_value: {}", crate::ast_dump(ob_value, None)?);

        // The context is Load, Store, etc. For some types of expressions such as Constants, it does not exist.
        let ctx: Option<String> = if let Ok(pyany) = ob_value.getattr("ctx") {
            pyany.get_type().extract().unwrap_or_default()
        } else { None };

        let expr_type = ob_value.get_type().name().expect(
            ob.error_message("<unknown>", format!("extracting type name {:?} in expression", ob_value).as_str()).as_str()
        );
        log::debug!("expression type: {}, value: {}", expr_type, crate::ast_dump(ob_value, None)?);
        let r = match expr_type {
            "Name" => {
                let name = Name::extract(ob_value).expect(
                    ob.error_message("<unknown>", format!("parsing Call expression {:?}", ob_value).as_str()).as_str()
                );
                Ok(Self{ctx: ctx, value: ExprType::Name(name)})
            }
            "Call" => {
                let et = Call::extract(ob_value).expect(
                    ob.error_message("<unknown>", format!("parsing Call expression {:?}", ob_value).as_str()).as_str()
                );
                Ok(Self{ctx: ctx, value: ExprType::Call(et)})
            },
            "Constant" => {
                let c = Constant::extract(ob_value)
                    .expect(
                        ob.error_message("<unknown>",
                            format!("extracting Constant in expression {:?}", crate::ast_dump(ob_value, None)?
                        ).as_str()).as_str()
                    );
                Ok(Self {
                    ctx: ctx,
                    value: ExprType::Constant(c)
                })
            },
            "List" => {
                //let list = crate::pytypes::List::<ExprType>::new();
                let list = Container::extract(ob_value).expect("extracting List");
                Ok(Self {
                    value: ExprType::List(list),
                    ctx: None,
                })
            }
            "UnaryOp" => {
                let c = UnaryOp::extract(ob_value)
                    .expect(
                        ob.error_message("<unknown>",
                            format!("extracting UnaryOp in expression {:?}", crate::ast_dump(ob_value, None)?
                        ).as_str()).as_str()
                    );
                Ok(Self {
                    ctx: ctx,
                    value: ExprType::UnaryOp(c)
                })

            },
            // In sitations where an expression is optional, we may see a NoneType expressions.
            "NoneType" => Ok(Expr{ctx: ctx, value: ExprType::NoneType(Constant(None))}),
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
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self.value {
            ExprType::Call(call) => {
                let name = format_ident!("{}", call.func.id);
                let mut arg_stream = proc_macro2::TokenStream::new();

                for s in call.args {
                    arg_stream.extend(s.clone().to_rust(ctx, options.clone(), symbols.clone()).expect(format!("parsing argument {:?}", s).as_str()));
                }
                Ok(quote!{#name(#arg_stream)})
            },
            ExprType::Constant(constant) => constant.to_rust(ctx, options, symbols),
            ExprType::UnaryOp(operand) => operand.to_rust(ctx, options, symbols),
            ExprType::Name(name) => name.to_rust(ctx, options, symbols),
            // NoneType expressions generate no code.
            ExprType::NoneType(_c) => Ok(quote!()),
            _ => {
                let error = CodeGenError(format!("Expr not implemented converting to Rust {:?}", self), None);
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
            }),
            ctx: None,
        };
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let tokens = expression.clone().to_rust(CodeGenContext::Module, options, symbols).unwrap();
        assert_eq!(tokens.to_string(), quote!(test()).to_string());
    }

}
