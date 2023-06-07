use std::default::Default;

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use pyo3::{PyAny, FromPyObject, PyResult};

pub mod statement;
pub use statement::*;
use statement::Statement;

pub mod function_def;
pub use function_def::*;
use function_def::FunctionDef;

pub mod arguments;
pub use arguments::*;
use arguments::{Arguments};

pub mod expression;
pub use expression::*;

pub mod import;
pub use import::*;
use import::{Import};

use crate::codegen::{CodeGen, PythonContext, Result};

use log::info;

#[derive(Clone, Debug)]
pub enum Type {
    Unimplemented,
}

impl<'a> FromPyObject<'a> for Type {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        info!("Type: {:?}", ob);
        Ok(Type::Unimplemented)
    }
}

/// Represents a module as imported from an ast.
#[derive(Clone, Debug, Default, FromPyObject)]
pub struct Module {
    pub body: Vec<Statement>,
    pub type_ignores: Vec<Type>,
}

impl CodeGen for Module {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        let mut stream = TokenStream::new();
        let stdpython = format_ident!("{}", ctx.stdpython);
        stream.extend(quote!(use #stdpython::*;));
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
        let mut ctx = PythonContext::default();
        let result = crate::parse("#test comment
def foo():
    continue
    pass
", "test_case").unwrap();
        info!("{:?}", result);
        //info!("{}", result);

        let code = result.to_rust(&mut ctx);
        info!("module: {:?}", code);
    }*/

    #[test]
    fn can_we_print() {
        let mut ctx = PythonContext::default();
        let result = crate::parse("#test comment
def foo():
    print(\"Test print.\")
", "test_case").unwrap();
        info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(&mut ctx);
        info!("module: {:?}", code);
    }

    /*
    #[test]
    fn can_we_import() {
        let result = crate::parse("import ast", "ast").unwrap();
        let mut ctx = PythonContext::default();
        info!("{:?}", result);

        let code = result.to_rust(&mut ctx);
        info!("module: {:?}", code);
    }*/

    /*#[test]
    fn can_we_import2() {
        let result = crate::parse("import ast.test as test", "ast").unwrap();
        let mut ctx = PythonContext::default();
        info!("{:?}", result);

        let code = result.to_rust(&mut ctx);
        info!("module: {:?}", code);
    }*/

}
