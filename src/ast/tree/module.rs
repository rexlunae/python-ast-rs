use std::default::Default;

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use pyo3::{PyAny, FromPyObject, PyResult};
use log::info;
use serde::{Serialize, Deserialize};

use crate::{
    Statement,
    CodeGen, PythonOptions, CodeGenContext,
    SymbolTableScopes,
};

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

/// Represents a module as imported from an ast. Because of some of the requirements of Python's
/// data module, we augment this by 
#[derive(Clone, Debug, Default, FromPyObject, Serialize, Deserialize)]
pub struct RawModule {
    pub body: Vec<Statement>,
    pub type_ignores: Vec<Type>,
}

/// Represents a module as imported from an ast.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]

pub struct Module {
    pub raw: RawModule,
    pub name: Option<crate::Name>,
    pub doc: Option<String>,
    pub filename: Option<String>,
}

impl<'a> FromPyObject<'a> for Module {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let raw_module = RawModule::extract(ob).expect("Failed parsing module.");

        Ok(Self {
            raw: raw_module,
            ..Default::default()
        })
    }
}

impl CodeGen for Module {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        symbols.new_scope();
        for s in self.raw.body {
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
        for s in self.raw.body {
            let statement = s.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())
                .expect(format!("parsing statement {:?} in module", s).as_str());
            if statement.to_string() != "" {
                stream.extend(statement);
            }
        }
        Ok(stream)
    }
}

impl crate::Object for Module {}


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

        let code = result.to_rust(CodeGenContext::Module("test_case".to_string()), options, SymbolTableScopes::new());
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_import() {
        let result = crate::parse("import ast", "ast").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(CodeGenContext::Module("test_case".to_string()), options, SymbolTableScopes::new());
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_import2() {
        let result = crate::parse("import ast.test as test", "ast").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(CodeGenContext::Module("test_case".to_string()), options, SymbolTableScopes::new());
        info!("module: {:?}", code);
    }

}
