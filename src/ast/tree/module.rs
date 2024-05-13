use std::{collections::HashMap, default::Default};

use log::info;
use proc_macro2::TokenStream;
use pyo3::{FromPyObject, PyAny, PyResult};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};

use crate::{CodeGen, CodeGenContext, Name, Object, PythonOptions, Statement, SymbolTableScopes};

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

/// Represents a module as imported from an ast. See the Module struct for the processed module.
#[derive(Clone, Debug, Default, FromPyObject, Serialize, Deserialize)]
pub struct RawModule {
    pub body: Vec<Statement>,
    pub type_ignores: Vec<Type>,
}

/// Represents a module as imported from an ast.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Module {
    pub raw: RawModule,
    pub name: Option<Name>,
    pub doc: Option<String>,
    pub filename: Option<String>,
    pub attributes: HashMap<Name, String>,
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

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut stream = TokenStream::new();
        let stdpython = format_ident!("{}", options.stdpython);
        if options.with_std_python {
            stream.extend(quote!(use #stdpython::*;));
        }
        for s in self.raw.body {
            let statement = s
                .clone()
                .to_rust(ctx.clone(), options.clone(), symbols.clone())
                .expect(format!("parsing statement {:?} in module", s).as_str());
            if statement.to_string() != "" {
                stream.extend(statement);
            }
        }
        Ok(stream)
    }
}

impl Object for Module {
    /// __dir__ is called to list the attributes of the object.
    fn __dir__(&self) -> Vec<impl AsRef<str>> {
        // XXX - Make this meaningful.
        vec![
            "__class__",
            "__class_getitem__",
            "__contains__",
            "__delattr__",
            "__delitem__",
            "__dir__",
            "__doc__",
            "__eq__",
            "__format__",
            "__ge__",
            "__getattribute__",
            "__getitem__",
            "__getstate__",
            "__gt__",
            "__hash__",
            "__init__",
            "__init_subclass__",
            "__ior__",
            "__iter__",
            "__le__",
            "__len__",
            "__lt__",
            "__ne__",
            "__new__",
            "__or__",
            "__reduce__",
            "__reduce_ex__",
            "__repr__",
            "__reversed__",
            "__ror__",
            "__setattr__",
            "__setitem__",
            "__sizeof__",
            "__str__",
            "__subclasshook__",
            "clear",
            "copy",
            "fromkeys",
            "get",
            "items",
            "keys",
            "pop",
            "popitem",
            "setdefault",
            "update",
            "values",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_we_print() {
        let options = PythonOptions::default();
        let result = crate::parse(
            "#test comment
def foo():
    print(\"Test print.\")
",
            "test_case.py",
        )
        .unwrap();
        info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(
            CodeGenContext::Module("test_case".to_string()),
            options,
            SymbolTableScopes::new(),
        );
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_import() {
        let result = crate::parse("import ast", "ast.py").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(
            CodeGenContext::Module("test_case".to_string()),
            options,
            SymbolTableScopes::new(),
        );
        info!("module: {:?}", code);
    }

    #[test]
    fn can_we_import2() {
        let result = crate::parse("import ast.test as test", "ast.py").unwrap();
        let options = PythonOptions::default();
        info!("{:?}", result);

        let code = result.to_rust(
            CodeGenContext::Module("test_case".to_string()),
            options,
            SymbolTableScopes::new(),
        );
        info!("module: {:?}", code);
    }
}
