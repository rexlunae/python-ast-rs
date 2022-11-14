use std::default::Default;
use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use pyo3::{PyAny, FromPyObject, PyResult};

pub mod statement;
pub use statement::*;
use statement::Statement;

pub mod function_def;
pub use function_def::*;
use function_def::FunctionDef;

pub mod arguments;
pub use arguments::*;
use arguments::{Arg, Arguments};

pub mod import;
pub use import::*;
use import::{Import};

use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};

#[derive(Clone, Debug)]
pub enum Type {
    Unimplemented,
}

impl<'a> FromPyObject<'a> for Type {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        println!("Type: {:?}", ob);
        Ok(Type::Unimplemented)
    }
}

#[derive(Clone, Debug, FromPyObject)]
pub struct Module {
    pub body: Vec<Statement>,
    pub type_ignores: Vec<Type>,
}

impl CodeGen for Module {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        let mut stream = TokenStream::new();
        for s in self.body.iter() {
            stream.extend(s.clone().to_rust(ctx)?);
        }
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn does_module_compile() {
        let result = crate::parse("#test comment
def foo():
    continue
    pass
", "test_case").unwrap();
        println!("{:?}", result);
        //println!("{}", result);

        let code = result.to_rust();
        println!("module: {:?}", code);
    }*/

    /*
    #[test]
    fn can_we_import() {
        let result = crate::parse("import ast", "ast").unwrap();
        let mut ctx = PythonContext::default();
        println!("{:?}", result);

        let code = result.to_rust(&mut ctx);
        println!("module: {:?}", code);
    }*/

    #[test]
    fn can_we_import2() {
        let result = crate::parse("import ast.test as test", "ast").unwrap();
        let mut ctx = PythonContext::default();
        println!("{:?}", result);

        let code = result.to_rust(&mut ctx);
        println!("module: {:?}", code);
    }

}
