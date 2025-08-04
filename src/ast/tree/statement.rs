use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::quote;

use crate::{
    dump, Assign, AugAssign, Call, ClassDef, CodeGen, CodeGenContext, Error, Expr, FunctionDef, Import,
    ImportFrom, Node, PythonOptions, SymbolTableScopes, If, For, While, Try, AsyncWith, AsyncFor, Raise, With,
};

use log::debug;

use serde::{Deserialize, Serialize};

/// AST node types that can be used as a statement implement this type.
pub trait PyStatementTrait: Clone + PartialEq {
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Statement {
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
    pub statement: StatementType,
}

impl<'a> FromPyObject<'a> for Statement {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        Ok(Self {
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
            statement: StatementType::extract_bound(ob)?,
        })
    }
}

impl Node for Statement {
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

impl CodeGen for Statement {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        self.statement.clone().find_symbols(symbols)
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        Ok(self
            .statement
            .clone()
            .to_rust(ctx, options, symbols)
            .expect(
                self.error_message(
                    "<unknown>",
                    format!("failed to compile statement {:#?}", self),
                )
                .as_str(),
            ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StatementType {
    AsyncFunctionDef(FunctionDef),
    Assign(Assign),
    AugAssign(AugAssign),
    Break,
    Continue,
    ClassDef(ClassDef),
    Call(Call),
    Pass,
    Return(Option<Expr>),
    Import(Import),
    ImportFrom(ImportFrom),
    Expr(Expr),
    FunctionDef(FunctionDef),
    If(If),
    For(For),
    While(While),
    Try(Try),
    AsyncWith(AsyncWith),
    AsyncFor(AsyncFor),
    Raise(Raise),
    With(With),

    Unimplemented(String),
}

impl<'a> FromPyObject<'a> for StatementType {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let err_msg = format!("getting type for statement {:?}", ob);
        let ob_type = ob
            .get_type()
            .name()
            .unwrap_or_else(|_| panic!("{}", ob.error_message("<unknown>", err_msg)));

        debug!("statement...ob_type: {}...{}", ob_type, dump(ob, Some(4))?);
        match ob_type.extract::<String>()?.as_str() {
            "AsyncFunctionDef" => Ok(StatementType::AsyncFunctionDef(
                FunctionDef::extract_bound(ob).unwrap_or_else(|_| {
                    panic!("Failed to extract async function: {:?}", dump(ob, Some(4)))
                }),
            )),
            "Assign" => {
                let assignment = Assign::extract_bound(ob).expect("reading assignment");
                Ok(StatementType::Assign(assignment))
            }
            "AugAssign" => {
                let aug_assignment = AugAssign::extract_bound(ob).expect("reading augmented assignment");
                Ok(StatementType::AugAssign(aug_assignment))
            }
            "Pass" => Ok(StatementType::Pass),
            "Call" => {
                let call =
                    Call::extract_bound(&ob.getattr("value").unwrap_or_else(|_| {
                        panic!("getting value from {:?} in call statement", ob)
                    }))
                    .unwrap_or_else(|_| panic!("extracting call statement {:?}", ob));
                debug!("call: {:?}", call);
                Ok(StatementType::Call(call))
            }
            "ClassDef" => Ok(StatementType::ClassDef(
                ClassDef::extract_bound(ob).unwrap_or_else(|_| panic!("Class definition {:?}", ob)),
            )),
            "Continue" => Ok(StatementType::Continue),
            "Break" => Ok(StatementType::Break),
            "FunctionDef" => Ok(StatementType::FunctionDef(
                FunctionDef::extract_bound(ob).unwrap_or_else(|_| {
                    panic!("Failed to extract function: {:?}", dump(ob, Some(4)))
                }),
            )),
            "Import" => Ok(StatementType::Import(
                Import::extract_bound(ob).unwrap_or_else(|_| panic!("Import {:?}", ob)),
            )),
            "ImportFrom" => Ok(StatementType::ImportFrom(
                ImportFrom::extract_bound(ob).unwrap_or_else(|_| panic!("ImportFrom {:?}", ob)),
            )),
            "Expr" => {
                let expr = ob.extract()
                    .expect(format!("Expr {:?}", ob).as_str());
                Ok(StatementType::Expr(expr))
            }
            "Return" => {
                log::debug!("return expression: {}", dump(ob, None)?);
                // Extract the return value from the Return statement's 'value' field
                let return_value = if let Ok(value_attr) = ob.getattr("value") {
                    if value_attr.is_none() {
                        // Bare 'return' statement - create a NoneType Expr
                        Some(Expr {
                            value: crate::tree::ExprType::NoneType(crate::tree::Constant(None)),
                            ctx: None,
                            lineno: ob.lineno(),
                            col_offset: ob.col_offset(),
                            end_lineno: ob.end_lineno(),
                            end_col_offset: ob.end_col_offset(),
                        })
                    } else {
                        // Return with actual expression - extract as ExprType then wrap in Expr
                        let expr_value: crate::tree::ExprType = value_attr.extract()
                            .unwrap_or_else(|_| panic!("return value ExprType {:?}", dump(&value_attr, None).unwrap_or_else(|_| "unknown".to_string())));
                        Some(Expr {
                            value: expr_value,
                            ctx: None,
                            lineno: ob.lineno(),
                            col_offset: ob.col_offset(),
                            end_lineno: ob.end_lineno(),
                            end_col_offset: ob.end_col_offset(),
                        })
                    }
                } else {
                    None
                };
                Ok(StatementType::Return(return_value))
            }
            "If" => {
                let if_stmt = If::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("If statement {:?}", dump(ob, None)));
                Ok(StatementType::If(if_stmt))
            }
            "For" => {
                let for_stmt = For::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("For statement {:?}", dump(ob, None)));
                Ok(StatementType::For(for_stmt))
            }
            "While" => {
                let while_stmt = While::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("While statement {:?}", dump(ob, None)));
                Ok(StatementType::While(while_stmt))
            }
            "Try" => {
                let try_stmt = Try::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("Try statement {:?}", dump(ob, None)));
                Ok(StatementType::Try(try_stmt))
            }
            "AsyncWith" => {
                let async_with_stmt = AsyncWith::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("AsyncWith statement {:?}", dump(ob, None)));
                Ok(StatementType::AsyncWith(async_with_stmt))
            }
            "AsyncFor" => {
                let async_for_stmt = AsyncFor::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("AsyncFor statement {:?}", dump(ob, None)));
                Ok(StatementType::AsyncFor(async_for_stmt))
            }
            "Raise" => {
                let raise_stmt = Raise::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("Raise statement {:?}", dump(ob, None)));
                Ok(StatementType::Raise(raise_stmt))
            }
            "With" => {
                let with_stmt = With::extract_bound(ob)
                    .unwrap_or_else(|_| panic!("With statement {:?}", dump(ob, None)));
                Ok(StatementType::With(with_stmt))
            }
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Unimplemented statement type {}, {}",
                ob_type,
                dump(ob, None)?
            ))),
        }
    }
}

impl CodeGen for StatementType {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        match self {
            StatementType::Assign(a) => a.find_symbols(symbols),
            StatementType::AugAssign(a) => a.find_symbols(symbols),
            StatementType::ClassDef(c) => c.find_symbols(symbols),
            StatementType::FunctionDef(f) => f.find_symbols(symbols),
            StatementType::Import(i) => i.find_symbols(symbols),
            StatementType::ImportFrom(i) => i.find_symbols(symbols),
            StatementType::Expr(e) => e.find_symbols(symbols),
            StatementType::If(i) => i.find_symbols(symbols),
            StatementType::For(f) => f.find_symbols(symbols),
            StatementType::While(w) => w.find_symbols(symbols),
            StatementType::Try(t) => t.find_symbols(symbols),
            StatementType::AsyncWith(aw) => aw.find_symbols(symbols),
            StatementType::AsyncFor(af) => af.find_symbols(symbols),
            StatementType::Raise(r) => r.find_symbols(symbols),
            StatementType::With(w) => w.find_symbols(symbols),
            _ => symbols,
        }
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            StatementType::AsyncFunctionDef(s) => {
                let func_def = s
                    .to_rust(Self::Context::Async(Box::new(ctx)), options, symbols)
                    .expect("Parsing async function");
                Ok(quote!(#func_def))
            }
            StatementType::Assign(a) => a.to_rust(ctx, options, symbols),
            StatementType::AugAssign(a) => a.to_rust(ctx, options, symbols),
            StatementType::Break => Ok(quote! {break;}),
            StatementType::Call(c) => c.to_rust(ctx, options, symbols),
            StatementType::ClassDef(c) => c.to_rust(ctx, options, symbols),
            StatementType::Continue => Ok(quote! {continue;}),
            StatementType::Pass => Ok(quote! {}),
            StatementType::FunctionDef(s) => s.to_rust(ctx, options, symbols),
            StatementType::Import(s) => s.to_rust(ctx, options, symbols),
            StatementType::ImportFrom(s) => s.to_rust(ctx, options, symbols),
            StatementType::Expr(s) => s.to_rust(ctx, options, symbols),
            StatementType::Return(None) => Ok(quote!(return)),
            StatementType::Return(Some(e)) => {
                let exp = e
                    .clone()
                    .to_rust(ctx, options, symbols)
                    .unwrap_or_else(|_| panic!("parsing expression {:#?}", e));
                Ok(quote!(return #exp))
            }
            StatementType::If(i) => i.to_rust(ctx, options, symbols),
            StatementType::For(f) => f.to_rust(ctx, options, symbols),
            StatementType::While(w) => w.to_rust(ctx, options, symbols),
            StatementType::Try(t) => t.to_rust(ctx, options, symbols),
            StatementType::AsyncWith(aw) => aw.to_rust(ctx, options, symbols),
            StatementType::AsyncFor(af) => af.to_rust(ctx, options, symbols),
            StatementType::Raise(r) => r.to_rust(ctx, options, symbols),
            StatementType::With(w) => w.to_rust(ctx, options, symbols),
            _ => {
                let error = Error::StatementNotYetImplemented(self);
                Err(Box::new(error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_pass_statement() {
        let statement = StatementType::Pass;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(
            CodeGenContext::Module("".to_string()),
            options,
            SymbolTableScopes::new(),
        );

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), true);
    }

    #[test]
    fn check_break_statement() {
        let statement = StatementType::Break;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(
            CodeGenContext::Module("".to_string()),
            options,
            SymbolTableScopes::new(),
        );

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn check_continue_statement() {
        let statement = StatementType::Continue;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(
            CodeGenContext::Module("".to_string()),
            options,
            SymbolTableScopes::new(),
        );

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn return_with_nothing() {
        let tree = crate::parse("return", "<none>").unwrap();
        assert_eq!(tree.raw.body.len(), 1);
        assert_eq!(
            tree.raw.body[0].statement,
            StatementType::Return(Some(Expr {
                value: crate::tree::ExprType::NoneType(crate::tree::Constant(None)),
                lineno: Some(1),
                col_offset: Some(0),
                end_lineno: Some(1),
                end_col_offset: Some(6),
                ..Default::default()
            }))
        );
    }

    #[test]
    fn return_with_expr() {
        let lit = litrs::Literal::Integer(litrs::IntegerLit::parse(String::from("8")).unwrap());
        let tree = crate::parse("return 8", "<none>").unwrap();
        assert_eq!(tree.raw.body.len(), 1);
        assert_eq!(
            tree.raw.body[0].statement,
            StatementType::Return(Some(Expr {
                value: crate::tree::ExprType::Constant(crate::tree::Constant(Some(lit))),
                lineno: Some(1),
                col_offset: Some(0),
                end_lineno: Some(1),
                end_col_offset: Some(8),
                ..Default::default()
            }))
        );
    }

    #[test]
    fn does_module_compile() {
        let options = PythonOptions::default();
        let result = crate::parse(
            "#test comment
def foo():
    continue
    pass
",
            "test_case",
        )
        .unwrap();
        log::info!("{:?}", result);
        let code = result.to_rust(
            CodeGenContext::Module("".to_string()),
            options,
            SymbolTableScopes::new(),
        );
        log::info!("module: {:?}", code);
    }
}
