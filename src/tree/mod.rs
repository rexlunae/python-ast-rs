use std::default::Default;

use crate::codegen::CodeGenContext;

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use pyo3::{PyAny, FromPyObject, PyResult};

pub mod statement;
pub use statement::*;

pub mod function_def;
pub use function_def::*;

pub mod arguments;
pub use arguments::*;

pub mod assign;
pub use assign::*;

pub mod bin_ops;
pub use bin_ops::*;

pub mod bool_ops;
pub use bool_ops::*;

pub mod call;
pub use call::*;

pub mod class_def;
pub use class_def::*;

pub mod compare;
pub use compare::*;

pub mod constant;
pub use constant::*;

pub mod expression;
pub use expression::*;

pub mod import;
pub use import::*;

pub mod keyword;
pub use keyword::*;

pub mod list;
pub use list::*;

pub mod parameters;
pub use parameters::*;

pub mod name;
pub use name::*;

pub mod unary_op;
pub use unary_op::*;

use crate::codegen::{CodeGen, PythonOptions};
use crate::symbols::SymbolTableScopes;

use log::info;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Default, FromPyObject, Serialize, Deserialize)]
pub struct Module {
    pub body: Vec<Statement>,
    pub type_ignores: Vec<Type>,
}

impl<'a> CodeGen for Module {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        symbols.new_scope();
        for s in self.body {
            symbols = s.clone().find_symbols(symbols);
        }
        symbols
    }

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut stream = TokenStream::new();
        let stdpython = format_ident!("{}", options.stdpython);
        if options.with_std_python {
            stream.extend(quote!(use #stdpython::*;));
        }
        for s in self.body {
            let statement = s.clone().to_rust(ctx, options.clone(), symbols.clone())
                .expect(format!("parsing statement {:?} in module", s).as_str());
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
    fn can_we_print() {
        let options = PythonOptions::default();
        let result = crate::parse("#test comment
def foo():
    print(\"Test print.\")
", "test_case").unwrap();
        info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(CodeGenContext::Module, options, SymbolTableScopes::new());
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_import() {
        let result = crate::parse("import ast", "ast").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(CodeGenContext::Module, options, SymbolTableScopes::new());
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_import2() {
        let result = crate::parse("import ast.test as test", "ast").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(CodeGenContext::Module, options, SymbolTableScopes::new());
        info!("module: {:?}", code);
    }

}
