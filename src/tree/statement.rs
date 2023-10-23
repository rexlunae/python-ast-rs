use pyo3::{PyAny, FromPyObject, PyResult};
use proc_macro2::TokenStream;
use quote::quote;

use crate::tree::{FunctionDef, Import, ImportFrom, Expr, Call, ClassDef};
use crate::codegen::{CodeGen, CodeGenError, PythonOptions, Node, CodeGenContext};

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
        let err_msg = format!("getting type for statement {:?}", ob);
        let ob_type = ob.get_type().name().expect(
            ob.error_message("<unknown>", err_msg.as_str()).as_str()
        );

        let lineno = ob.lineno();
        println!("statement line number {:?}", lineno);

        let col_offset = ob.col_offset();
        println!("statement col offset {:?}", col_offset);

        let end_lineno = ob.end_lineno();
        println!("statement end line number {:?}", end_lineno);

        let end_col_offset = ob.end_col_offset();
        println!("statement end col offset {:?}", end_col_offset);

        debug!("statement ob_type: {}...{}", ob_type, crate::ast_dump(ob, Some(4))?);
        match ob_type {
            "Pass" => Ok(Statement::Pass),
            "Call" => {
                let call = Call::extract(
                    ob.getattr("value").expect(format!("getting value from {:?} in call statement", ob).as_str())
                ).expect(format!("extracting call statement {:?}", ob).as_str());
                debug!("call: {:?}", call);
                Ok(Statement::Call(call))
        },
            "ClassDef" => Ok(Statement::ClassDef(ClassDef::extract(ob).expect(format!("Class definition {:?}", ob).as_str()))),
            "Continue" => Ok(Statement::Continue),
            "Break" => Ok(Statement::Break),
            "FunctionDef" => Ok(Statement::FunctionDef(FunctionDef::extract(ob).expect(format!("Function definition {:?}", ob).as_str()))),
            "Import" => Ok(Statement::Import(Import::extract(ob).expect(format!("Import {:?}", ob).as_str()))),
            "ImportFrom" => Ok(Statement::ImportFrom(ImportFrom::extract(ob).expect(format!("ImportFrom {:?}", ob).as_str()))),
            "Expr" => {
                let expr = Expr::extract(
                    ob.extract().expect(format!("extracting Expr {:?}", ob).as_str())
                ).expect(format!("Expr {:?}", ob).as_str());
                Ok(Statement::Expr(expr))
            },
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!("Unimplemented statement type {}, {}", ob_type, crate::ast_dump(ob, None)?)))
        }
    }
}

impl<'a> CodeGen for Statement {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
        debug!("generating statement: {:?}", self);
        match self {
            Statement::Break => Ok(quote!{break;}),
            Statement::ClassDef(c) => c.to_rust(ctx, options),
            Statement::Continue => Ok(quote!{continue;}),
            Statement::Pass => Ok(quote!{}),
            Statement::FunctionDef(s) => s.to_rust(ctx, options),
            Statement::Import(s) => s.to_rust(ctx, options),
            Statement::ImportFrom(s) => s.to_rust(ctx, options),
            Statement::Expr(s) => s.to_rust(ctx, options),
            _ => {
                let error = CodeGenError(format!("Statement not implemented {:?}", self), None);
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
        let statement = Statement::Pass;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(CodeGenContext::Module, options);

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), true);
    }

    #[test]
    fn check_break_statement() {
        let statement = Statement::Break;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(CodeGenContext::Module, options);

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn check_continue_statement() {
        let statement = Statement::Continue;
        let options = PythonOptions::default();
        let tokens = statement.clone().to_rust(CodeGenContext::Module, options);

        debug!("statement: {:?}, tokens: {:?}", statement, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }
}
