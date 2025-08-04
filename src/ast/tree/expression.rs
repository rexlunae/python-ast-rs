use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    dump, Attribute, Await, BinOp, BoolOp, Call, CodeGen, CodeGenContext, Compare, Constant, Error,
    Name, NamedExpr, Node, PythonOptions, SymbolTableScopes, UnaryOp, Lambda, IfExp, Dict, Set, Tuple, Subscript,
};

/// Mostly this shouldn't be used, but it exists so that we don't have to manually implement FromPyObject on all of ExprType
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[repr(transparent)]
pub struct Container<T>(pub T);

impl<'a> FromPyObject<'a> for Container<crate::pytypes::List<ExprType>> {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let list = crate::pytypes::List::<ExprType>::new();

        log::debug!("pylist: {}", dump(ob, Some(4))?);
        let _converted_list: Vec<Bound<PyAny>> = ob.extract()?;
        for item in _converted_list.iter() {
            log::debug!("item: {:?}", item);
        }

        Ok(Self(list))
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum ExprType {
    BoolOp(BoolOp),
    NamedExpr(NamedExpr),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
    Lambda(Lambda),
    IfExp(IfExp),
    Dict(Dict),
    Set(Set),
    /*ListComp(ListComp),
    SetComp(SetComp),
    DictComp(DictComp),
    GeneratorExp(),*/
    Await(Await),
    /*Yield(),
    YieldFrom(),*/
    Compare(Compare),
    Call(Call),
    /*FormattedValue(),
    JoinedStr(),*/
    Constant(Constant),

    /// These can appear in a few places, such as the left side of an assignment.
    Attribute(Attribute),
    Subscript(Subscript),
    /*Starred(),*/
    Name(Name),
    List(Vec<ExprType>),
    Tuple(Tuple),
    /*Slice(),*/
    NoneType(Constant),

    Unimplemented(String),
    #[default]
    Unknown,
}

impl<'a> FromPyObject<'a> for ExprType {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        log::debug!("exprtype ob: {}", dump(ob, Some(4))?);

        let expr_type = ob.get_type().name().expect(
            ob.error_message(
                "<unknown>",
                format!("extracting type name {:?} in expression", dump(ob, None)),
            )
            .as_str(),
        );
        log::debug!("expression type: {}, value: {}", expr_type, dump(ob, None)?);

        let r = match expr_type.extract::<String>()?.as_str() {
            "Attribute" => {
                let a = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Attribute in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Attribute(a))
            }
            "Await" => {
                //println!("await: {}", dump(ob, None)?);
                let a = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting await value in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Await(a))
            }
            "Call" => {
                let et = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("parsing Call expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Call(et))
            }
            "Compare" => {
                let c = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Compare in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Compare(c))
            }
            "Constant" => {
                log::debug!("constant: {}", dump(ob, None)?);
                let c = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Constant in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Constant(c))
            }
            "List" => {
                //let list = crate::pytypes::List::<ExprType>::new();
                let list: Vec<ExprType> = ob
                    .extract()
                    .expect(format!("extracting List {}", dump(ob, None)?).as_str());
                Ok(Self::List(list))
            }
            "Name" => {
                let name = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("parsing Name expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Name(name))
            }
            "UnaryOp" => {
                let c = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting UnaryOp in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::UnaryOp(c))
            }
            "BinOp" => {
                let c = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting BinOp in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::BinOp(c))
            }
            "Lambda" => {
                let l = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Lambda in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Lambda(l))
            }
            "IfExp" => {
                let i = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting IfExp in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::IfExp(i))
            }
            "Dict" => {
                let d = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Dict in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Dict(d))
            }
            "Set" => {
                let s = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Set in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Set(s))
            }
            "Tuple" => {
                let t = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Tuple in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Tuple(t))
            }
            "Subscript" => {
                let s = ob.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting Subscript in expression {}", dump(ob, None)?),
                    )
                    .as_str(),
                );
                Ok(Self::Subscript(s))
            }
            _ => {
                let err_msg = format!(
                    "Unimplemented expression type {}, {}",
                    expr_type,
                    dump(ob, None)?
                );
                Err(pyo3::exceptions::PyValueError::new_err(
                    ob.error_message("<unknown>", err_msg.as_str()),
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

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            ExprType::Attribute(attribute) => attribute.to_rust(ctx, options, symbols),
            ExprType::Await(func) => func.to_rust(ctx, options, symbols),
            ExprType::BinOp(binop) => binop.to_rust(ctx, options, symbols),
            ExprType::Call(call) => call.to_rust(ctx, options, symbols),
            ExprType::Compare(c) => c.to_rust(ctx, options, symbols),
            ExprType::Constant(c) => c.to_rust(ctx, options, symbols),
            ExprType::Lambda(l) => l.to_rust(ctx, options, symbols),
            ExprType::IfExp(i) => i.to_rust(ctx, options, symbols),
            ExprType::Dict(d) => d.to_rust(ctx, options, symbols),
            ExprType::Set(s) => s.to_rust(ctx, options, symbols),
            ExprType::Tuple(t) => t.to_rust(ctx, options, symbols),
            ExprType::Subscript(s) => s.to_rust(ctx, options, symbols),
            ExprType::List(l) => {
                let mut ts = TokenStream::new();
                for li in l {
                    let code = li
                        .clone()
                        .to_rust(ctx.clone(), options.clone(), symbols.clone())
                        .expect(format!("Extracting list item {:?}", li).as_str());
                    ts.extend(code);
                    ts.extend(quote!(,));
                }
                Ok(ts)
            }
            ExprType::Name(name) => name.to_rust(ctx, options, symbols),
            ExprType::NoneType(c) => c.to_rust(ctx, options, symbols),
            ExprType::UnaryOp(operand) => operand.to_rust(ctx, options, symbols),

            _ => {
                let error = Error::ExprTypeNotYetImplemented(self);
                Err(error.into())
            }
        }
    }
}

/// An Expr only contains a single value key, which leads to the actual expression,
/// which is one of several types.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Expr {
    pub value: ExprType,
    pub ctx: Option<String>,
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

impl<'a> FromPyObject<'a> for Expr {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        let err_msg = format!("extracting object value {} in expression", dump(ob, None)?);

        let ob_value = ob
            .getattr("value")
            .expect(ob.error_message("<unknown>", err_msg.as_str()).as_str());
        log::debug!("ob_value: {}", dump(&ob_value, None)?);

        // The context is Load, Store, etc. For some types of expressions such as Constants, it does not exist.
        let ctx: Option<String> = if let Ok(pyany) = ob_value.getattr("ctx") {
            pyany.get_type().extract().unwrap_or_default()
        } else {
            None
        };

        let mut r = Self {
            value: ExprType::Unknown,
            ctx: ctx,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        };

        let expr_type = ob_value.get_type().name().expect(
            ob.error_message(
                "<unknown>",
                format!("extracting type name {:?} in expression", ob_value),
            )
            .as_str(),
        );
        log::debug!(
            "expression type: {}, value: {}",
            expr_type,
            dump(&ob_value, None)?
        );
        match expr_type.extract::<String>()?.as_str() {
            "Atribute" => {
                let a = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting BinOp in expression {:?}", dump(&ob_value, None)?),
                    )
                    .as_str(),
                );
                r.value = ExprType::Attribute(a);
                Ok(r)
            }
            "Await" => {
                let a = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting BinOp in expression {:?}", dump(&ob_value, None)?),
                    )
                    .as_str(),
                );
                r.value = ExprType::Await(a);
                Ok(r)
            }
            "BinOp" => {
                let c = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting BinOp in expression {:?}", dump(&ob_value, None)?),
                    )
                    .as_str(),
                );
                r.value = ExprType::BinOp(c);
                Ok(r)
            }
            "BoolOp" => {
                let c = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("extracting BinOp in expression {:?}", dump(&ob_value, None)?),
                    )
                    .as_str(),
                );
                r.value = ExprType::BoolOp(c);
                Ok(r)
            }
            "Call" => {
                let et = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("parsing Call expression {:?}", ob_value),
                    )
                    .as_str(),
                );
                r.value = ExprType::Call(et);
                Ok(r)
            }
            "Constant" => {
                let c = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting Constant in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::Constant(c);
                Ok(r)
            }
            "Compare" => {
                let c = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting Compare in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::Compare(c);
                Ok(r)
            }
            "List" => {
                //let list = crate::pytypes::List::<ExprType>::new();
                let list: Vec<ExprType> = ob.extract().expect("extracting List");
                r.value = ExprType::List(list);
                Ok(r)
            }
            "Name" => {
                let name = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!("parsing Call expression {:?}", ob_value),
                    )
                    .as_str(),
                );
                r.value = ExprType::Name(name);
                Ok(r)
            }
            "UnaryOp" => {
                let c = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting UnaryOp in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::UnaryOp(c);
                Ok(r)
            }
            "Lambda" => {
                let l = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting Lambda in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::Lambda(l);
                Ok(r)
            }
            "IfExp" => {
                let i = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting IfExp in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::IfExp(i);
                Ok(r)
            }
            "Dict" => {
                let d = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting Dict in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::Dict(d);
                Ok(r)
            }
            "Set" => {
                let s = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting Set in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::Set(s);
                Ok(r)
            }
            "Tuple" => {
                let t = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting Tuple in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::Tuple(t);
                Ok(r)
            }
            "Subscript" => {
                let s = ob_value.extract().expect(
                    ob.error_message(
                        "<unknown>",
                        format!(
                            "extracting Subscript in expression {:?}",
                            dump(&ob_value, None)?
                        ),
                    )
                    .as_str(),
                );
                r.value = ExprType::Subscript(s);
                Ok(r)
            }
            // In sitations where an expression is optional, we may see a NoneType expressions.
            "NoneType" => {
                r.value = ExprType::NoneType(Constant(None));
                Ok(r)
            }
            _ => {
                let err_msg = format!(
                    "Unimplemented expression type {}, {}",
                    expr_type,
                    dump(ob, None)?
                );
                Err(pyo3::exceptions::PyValueError::new_err(
                    ob.error_message("<unknown>", err_msg.as_str()),
                ))
            }
        }
    }
}

impl CodeGen for Expr {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        let module_name = match ctx.clone() {
            CodeGenContext::Module(name) => name,
            _ => "unknown".to_string(),
        };

        match self.value.clone() {
            ExprType::Await(a) => a.to_rust(ctx.clone(), options, symbols),
            ExprType::BinOp(binop) => binop.to_rust(ctx.clone(), options, symbols),
            ExprType::BoolOp(boolop) => boolop.to_rust(ctx.clone(), options, symbols),
            ExprType::Call(call) => call.to_rust(ctx.clone(), options, symbols),
            ExprType::Constant(constant) => constant.to_rust(ctx, options, symbols),
            ExprType::Compare(compare) => compare.to_rust(ctx, options, symbols),
            ExprType::Lambda(l) => l.to_rust(ctx, options, symbols),
            ExprType::IfExp(i) => i.to_rust(ctx, options, symbols),
            ExprType::Dict(d) => d.to_rust(ctx, options, symbols),
            ExprType::Set(s) => s.to_rust(ctx, options, symbols),
            ExprType::Tuple(t) => t.to_rust(ctx, options, symbols),
            ExprType::Subscript(s) => s.to_rust(ctx, options, symbols),
            ExprType::UnaryOp(operand) => operand.to_rust(ctx, options, symbols),
            ExprType::Name(name) => name.to_rust(ctx, options, symbols),
            // NoneType expressions generate no code.
            ExprType::NoneType(_c) => Ok(quote!()),
            _ => {
                let error = Error::ExprTypeNotYetImplemented(self.value);
                Err(error.into())
            }
        }
    }
}

impl Node for Expr {
    fn lineno(&self) -> Option<usize> {
        self.lineno
    }

    fn col_offset(&self) -> Option<usize> {
        self.col_offset
    }

    fn end_lineno(&self) -> Option<usize> {
        self.end_lineno
    }

    fn end_col_offset(&self) -> Option<usize> {
        self.end_col_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_call_expression() {
        let expression = crate::parse("test()", "test.py").unwrap();
        println!("Python tree: {:#?}", expression);
        let mut options = PythonOptions::default();
        options.with_std_python = false;
        let symbols = SymbolTableScopes::new();
        let tokens = expression
            .clone()
            .to_rust(CodeGenContext::Module("test".to_string()), options, symbols)
            .unwrap();
        println!("Rust tokens: {}", tokens.to_string());
        assert_eq!(tokens.to_string(), quote!(test()).to_string());
    }
}
