use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{Arg, CodeGen, CodeGenContext, ExprType, Keyword, PythonOptions, SymbolTableScopes};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Call {
    pub func: Box<ExprType>,
    pub args: Vec<Arg>,
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
        let name = self
            .func
            .to_rust(ctx.clone(), options.clone(), symbols.clone())
            .expect("Call.func");
        // XXX - How are we going to figure out the parameter list?
        //let symbol = symbols.get(&self.func.id).expect(format!("looking up function {}", self.func.id).as_str());
        //println!("symbol: {:?}", symbol);
        let mut args = TokenStream::new();
        for arg in self.args {
            let arg = arg
                .clone()
                .to_rust(ctx.clone(), options.clone(), symbols.clone())
                .expect(format!("Call.args {:?}", arg).as_str());
            args.extend(arg);
            args.extend(quote!(,));
        }
        Ok(quote!(#name(#args)))
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
