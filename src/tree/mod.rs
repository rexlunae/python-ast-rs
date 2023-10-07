use std::default::Default;

use crate::codegen::CodeGenContext;

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use pyo3::{PyAny, FromPyObject, PyResult};

use log::debug;

pub mod statement;
pub use statement::*;

pub mod function_def;
pub use function_def::*;

pub mod arguments;
pub use arguments::*;

pub mod call;
pub use call::*;

pub mod class_def;
pub use class_def::*;

pub mod constant;
pub use constant::*;

pub mod expression;
pub use expression::*;

pub mod import;
pub use import::*;

pub mod parameters;
pub use parameters::*;

pub mod name;
pub use name::*;

use crate::codegen::{CodeGen, PythonOptions};

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

impl<'a> CodeGen for Module {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, ctx: Self::Context, options: Self::Options) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut stream = TokenStream::new();
        let stdpython = format_ident!("{}", options.stdpython);
        stream.extend(quote!(use #stdpython::*;));
        for s in self.body {
            let statement = s.clone().to_rust(ctx, options.clone())?;
            debug!("{:?}, {}", s, statement);
            if statement.to_string() != "" {
                stream.extend(statement);
            }
        }
        Ok(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn does_module_compile() {
        let options = PythonOptions::default();
        let result = crate::parse("#test comment
def foo():
    continue
    pass
", "test_case").unwrap();
        info!("{:?}", result);
        let code = result.to_rust(CodeGenContext::Module, options);
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_print() {
        let options = PythonOptions::default();
        let result = crate::parse("#test comment
def foo():
    print(\"Test print.\")
", "test_case").unwrap();
        info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(CodeGenContext::Module, options);
        info!("module: {:?}", code);
    }


    #[test]
    fn can_we_import() {
        let result = crate::parse("import ast", "ast").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(CodeGenContext::Module, options);
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_import2() {
        let result = crate::parse("import ast.test as test", "ast").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(CodeGenContext::Module, options);
        info!("module: {:?}", code);
    }

}
