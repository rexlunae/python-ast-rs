use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{CodeGen, CodeGenContext, ExprType, Keyword, PythonOptions, SymbolTableScopes};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Call {
    pub func: Box<ExprType>,
    pub args: Vec<ExprType>,
    pub keywords: Vec<Keyword>,
}

impl<'a> FromPyObject<'a> for Call {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let func = ob.getattr("func").expect("Call.func");
        let args = ob.getattr("args").expect("Call.args");
        let keywords = ob.getattr("keywords").expect("Call.keywords");
        Ok(Call {
            func: Box::new(func.extract().expect("Call.func")),
            args: args.extract().expect("Call.args"),
            keywords: keywords.extract().expect("Call.keywords"),
        })
    }
}

impl<'a> CodeGen for Call {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let name = self.func.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        
        let mut all_args = Vec::new();
        
        // Add positional arguments
        for arg in self.args {
            let rust_arg = arg.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            all_args.push(rust_arg);
        }
        
        // Add keyword arguments
        for keyword in self.keywords {
            let rust_kw = keyword.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            all_args.push(rust_kw);
        }
        
        Ok(quote!(#name(#(#all_args),*)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_of_function() {
        let options = PythonOptions::default();
        let result = crate::parse(
            "def foo(a = 7):
    pass

foo(b=9)",
            "test.py",
        )
        .unwrap();
        println!("Python tree: {:#?}", result);
        let code = result
            .to_rust(
                CodeGenContext::Module("test".to_string()),
                options,
                SymbolTableScopes::new(),
            )
            .unwrap();
        println!("Rust code: {}", code);
    }
}
