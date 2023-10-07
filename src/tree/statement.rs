use pyo3::{PyAny, FromPyObject, PyResult};
use proc_macro2::TokenStream;
use quote::quote;

use crate::tree::{FunctionDef, Import, ImportFrom, Expr, ExprType, Call, ClassDef};
use crate::codegen::{CodeGen, CodeGenError, PythonContext};

use log::debug;

#[derive(Clone, Debug)]
pub enum Statement {
    Break,
    Continue,
    ClassDef(ClassDef),
    Call(Call),
    Pass,
    Import(Import),
    ImportFrom(ImportFrom),
    Expr(Expr),
    FunctionDef(FunctionDef),

    Unimplemented(String),
}

impl<'a> FromPyObject<'a> for Statement {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let ob_type = ob.get_type().name()?;
        debug!("statement ob_type: {}...{}", ob_type, crate::ast_dump(ob, Some(4))?);
        match ob_type {
            "Pass" => Ok(Statement::Pass),
            "Call" => {
                let call = Call::extract(ob.getattr("value")?)?;
                debug!("call: {:?}", call);
                Ok(Statement::Call(call))
        },
            "ClassDef" => Ok(Statement::ClassDef(ClassDef::extract(ob)?)),
            "Continue" => Ok(Statement::Continue),
            "Break" => Ok(Statement::Break),
            "FunctionDef" => Ok(Statement::FunctionDef(FunctionDef::extract(ob)?)),
            "Import" => Ok(Statement::Import(Import::extract(ob)?)),
            "ImportFrom" => Ok(Statement::ImportFrom(ImportFrom::extract(ob)?)),
            "Expr" => {
                let expr = Expr::extract(ob.extract()?)?;
                Ok(Statement::Expr(expr))
            },
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unimplemented statement type {}, {}", ob_type, crate::ast_dump(ob, None)?)))
        }
    }
}

impl<'a> CodeGen for Statement {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        debug!("generating statement: {:?}", self);
        match self {
            Statement::Break => Ok(quote!{break;}),
            Statement::ClassDef(c) => c.to_rust(ctx),
            Statement::Continue => Ok(quote!{continue;}),
            Statement::Pass => Ok(quote!{}),
            Statement::FunctionDef(s) => s.to_rust(ctx),
            Statement::Import(s) => s.to_rust(ctx),
            Statement::ImportFrom(s) => s.to_rust(ctx),
            Statement::Expr(s) => s.to_rust(ctx),
            _ => {
                let error = CodeGenError(format!("Statement not implemented {:?}", self), None);
                Err(Box::new(error))
            }
        }
    }

    /// Override the base trait method.
    fn to_rust_trait_member(&self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let shaddow_self = (*self).clone();

        match shaddow_self {
            Statement::Pass => {
                return Ok(TokenStream::new())
            }
            Statement::Expr(e) => {
                return e.to_rust_trait_member(ctx)
            }
            Statement::FunctionDef(f) => {
                return f.to_rust_trait_member(ctx)
            }
            _ => {}
        };
        Err(Box::new(CodeGenError(format!("Unsupported trait member: {:?}", &self), None)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_pass_statement() {
        let statement = Statement::Pass;
        let mut ctx = PythonContext::default();
        let tokens = statement.clone().to_rust(&mut ctx);

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), true);
    }

    #[test]
    fn check_break_statement() {
        let statement = Statement::Break;
        let mut ctx = PythonContext::default();
        let tokens = statement.clone().to_rust(&mut ctx);

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn check_continue_statement() {
        let statement = Statement::Continue;
        let mut ctx = PythonContext::default();
        let tokens = statement.clone().to_rust(&mut ctx);

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }
}
