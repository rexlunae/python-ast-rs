use pyo3::{PyAny, FromPyObject, PyResult};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    dump,
    Node,
    Assign, FunctionDef, Import, ImportFrom, Expr, Call, ClassDef,
    CodeGen, CodeGenError, PythonOptions, CodeGenContext,
    SymbolTableScopes,
};

use log::debug;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Statement {
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
    pub statement: StatementType,
}

impl<'a> FromPyObject<'a> for Statement {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        Ok(Self {
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
            statement: StatementType::extract(ob)?,
        })
    }
}

impl<'a> Node<'a> for Statement {
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

impl<'a> CodeGen for Statement {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        self.statement.clone().find_symbols(symbols)
    }

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        Ok(self.statement.clone().to_rust(ctx, options, symbols).expect(
            self.error_message("<unknown>", format!("failed to compile statement {:#?}", self).as_str()).as_str()
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StatementType {
    Assign(Assign),
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

    Unimplemented(String),
}

impl<'a> FromPyObject<'a> for StatementType {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let err_msg = format!("getting type for statement {:?}", ob);
        let ob_type = ob.get_type().name().expect(
            ob.error_message("<unknown>", err_msg.as_str()).as_str()
        );

        debug!("statement...ob_type: {}...{}", ob_type, dump(ob, Some(4))?);
        match ob_type {
            "Assign" => {
                let assignment = Assign::extract(ob).expect("reading assignment");
                Ok(StatementType::Assign(assignment))
            },
            "Pass" => Ok(StatementType::Pass),
            "Call" => {
                let call = Call::extract(
                    ob.getattr("value").expect(format!("getting value from {:?} in call statement", ob).as_str())
                ).expect(format!("extracting call statement {:?}", ob).as_str());
                debug!("call: {:?}", call);
                Ok(StatementType::Call(call))
            },
            "ClassDef" => Ok(StatementType::ClassDef(ClassDef::extract(ob).expect(format!("Class definition {:?}", ob).as_str()))),
            "Continue" => Ok(StatementType::Continue),
            "Break" => Ok(StatementType::Break),
            "FunctionDef" => Ok(StatementType::FunctionDef(FunctionDef::extract(ob).expect(format!("Failed to extract function: {}", dump(ob, Some(4))?).as_str()))),
            "Import" => Ok(StatementType::Import(Import::extract(ob).expect(format!("Import {:?}", ob).as_str()))),
            "ImportFrom" => Ok(StatementType::ImportFrom(ImportFrom::extract(ob).expect(format!("ImportFrom {:?}", ob).as_str()))),
            "Expr" => {
                let expr = Expr::extract(
                    ob.extract().expect(format!("extracting Expr {:?}", ob).as_str())
                ).expect(format!("Expr {:?}", ob).as_str());
                Ok(StatementType::Expr(expr))
            },
            "Return" => {
                log::debug!("return expression: {}", dump(ob, None)?);
                let expr = Expr::extract(
                    ob.extract().expect(format!("extracting return Expr {:?}", ob).as_str())
                ).expect(format!("return Expr {}", dump(ob, None)?).as_str());
                Ok(StatementType::Return(Some(expr)))
            },
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unimplemented statement type {}, {}", ob_type, dump(ob, None)?)))
        }
    }
}

impl<'a> CodeGen for StatementType {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        match self {
            StatementType::Assign(a) => a.find_symbols(symbols),
            StatementType::ClassDef(c) => c.find_symbols(symbols),
            StatementType::FunctionDef(f) => f.find_symbols(symbols),
            StatementType::Import(i) => i.find_symbols(symbols),
            StatementType::ImportFrom(i) => i.find_symbols(symbols),
            StatementType::Expr(e) => e.find_symbols(symbols),
            _ => symbols
        }
    }

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        match self {
            StatementType::Assign(a) => a.to_rust(ctx, options, symbols),
            StatementType::Break => Ok(quote!{break;}),
            StatementType::Call(c) => c.to_rust(ctx, options, symbols),
            StatementType::ClassDef(c) => c.to_rust(ctx, options, symbols),
            StatementType::Continue => Ok(quote!{continue;}),
            StatementType::Pass => Ok(quote!{}),
            StatementType::FunctionDef(s) => s.to_rust(ctx, options, symbols),
            StatementType::Import(s) => s.to_rust(ctx, options, symbols),
            StatementType::ImportFrom(s) => s.to_rust(ctx, options, symbols),
            StatementType::Expr(s) => s.to_rust(ctx, options, symbols),
            StatementType::Return(None) => Ok(quote!(return)),
            StatementType::Return(Some(e)) => {
                let exp = e.clone().to_rust(ctx, options, symbols)
                    .expect(format!("parsing expression {:#?}", e).as_str());

                Ok(quote!(return #exp))
            },
            _ => {
                let error = CodeGenError::NotYetImplemented(format!("StatementType not implemented {:?}", self));
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
        let tokens = statement.clone().to_rust(CodeGenContext::Module("".to_string()), options, SymbolTableScopes::new());

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), true);
    }

    #[test]
    fn check_break_statement() {
        let statement = StatementType::Break;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(CodeGenContext::Module("".to_string()), options, SymbolTableScopes::new());

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn check_continue_statement() {
        let statement = StatementType::Continue;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(CodeGenContext::Module("".to_string()), options, SymbolTableScopes::new());

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn return_with_nothing() {
        let tree = crate::parse("return", "<none>").unwrap();
        assert_eq!(tree.body.len(), 1);
        assert_eq!(tree.body[0].statement, StatementType::Return(Some(Expr{value: crate::tree::ExprType::NoneType(crate::tree::Constant(None)),
            lineno: Some(1),
            col_offset: Some(0),
            end_lineno: Some(1),
            end_col_offset: Some(6),
            ..Default::default()
        })));
    }

    #[test]
    fn return_with_expr() {
        let lit = litrs::Literal::Integer(litrs::IntegerLit::parse(String::from("8")).unwrap());
        let tree = crate::parse("return 8", "<none>").unwrap();
        assert_eq!(tree.body.len(), 1);
        assert_eq!(tree.body[0].statement, StatementType::Return(Some(
            Expr{value: crate::tree::ExprType::Constant(
                crate::tree::Constant(Some(
                    lit
                ))),
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
        let result = crate::parse("#test comment
def foo():
    continue
    pass
", "test_case").unwrap();
        log::info!("{:?}", result);
        let code = result.to_rust(CodeGenContext::Module("".to_string()), options, SymbolTableScopes::new());
        log::info!("module: {:?}", code);
    }

}
