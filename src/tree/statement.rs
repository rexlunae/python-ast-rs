use pyo3::{PyAny, FromPyObject, PyResult};
use proc_macro2::TokenStream;
use quote::quote;

use crate::tree::{FunctionDef, Import, ImportFrom};
use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};

// This is just a way of extracting type information from Pyo3.
#[derive(Clone, Debug, FromPyObject)]
struct GenericStatement {
    pub __doc__: String,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Break,
    Continue,
    Pass,
    Import(Import),
    ImportFrom(ImportFrom),
    FunctionDef(FunctionDef),

    Unimplemented(String),
}

impl<'a> FromPyObject<'a> for Statement {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let gen_statement = GenericStatement::extract(ob)?;
        let parts: Vec<&str> = gen_statement.__doc__.split("(").collect();

        match parts[0] {
            "Pass" => Ok(Statement::Pass),
            "Continue" => Ok(Statement::Continue),
            "Break" => Ok(Statement::Break),
            "FunctionDef" => Ok(Statement::FunctionDef(FunctionDef::extract(ob)?)),
            "Import" => Ok(Statement::Import(Import::extract(ob)?)),
            "ImportFrom" => Ok(Statement::ImportFrom(ImportFrom::extract(ob)?)),
            _ => Ok(Statement::Unimplemented(String::from(parts[0]))),
        }

    }
}

impl CodeGen for Statement {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        match self {
            Statement::Break => Ok(quote!{break;}),
            Statement::Continue => Ok(quote!{continue;}),
            Statement::Pass => Ok(quote!{}),
            Statement::FunctionDef(s) => s.to_rust(ctx),
            Statement::Import(s) => s.to_rust(ctx),
            Statement::ImportFrom(s) => s.to_rust(ctx),
            _ => Err(CodeGenError(format!("Statement not implemented {:?}", self), None))
        }
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

        println!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), true);
    }

    #[test]
    fn check_break_statement() {
        let statement = Statement::Break;
        let mut ctx = PythonContext::default();
        let tokens = statement.clone().to_rust(&mut ctx);

        println!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn check_continue_statement() {
        let statement = Statement::Continue;
        let mut ctx = PythonContext::default();
        let tokens = statement.clone().to_rust(&mut ctx);

        println!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

}
