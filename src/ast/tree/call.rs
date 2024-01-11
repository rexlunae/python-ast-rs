use pyo3::{FromPyObject};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use serde::{Serialize, Deserialize};

use crate::{
    CodeGen, PythonOptions, CodeGenContext,
    Arg, Name, Keyword,
    SymbolTableScopes,
};

#[derive(Clone, Debug, Default, FromPyObject, Serialize, Deserialize, PartialEq)]
pub struct Call {
    pub func: Name,
    pub args: Vec<Arg>,
    pub keywords: Vec<Keyword>,
}

/*
impl<'a> FromPyObject<'a> for Call {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        debug!("100000000 {}", dump(ob, Some(2))?);
        let func = ob.getattr("func")?;
        debug!("110000000 {}", dump(func, Some(2))?);
        let func_name = Name::extract(func)?;
        debug!("200000000");
        debug!("300000000");
        Err(pyo3::exceptions::PyValueError::new_err(format!("Unimplemented call {}...{}", ob, dump(ob, None)?)))
    }
}
*/

impl<'a> CodeGen for Call {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let name = format_ident!("{}", self.func.id);
        // XXX - How are we going to figure out the parameter list?
        let symbol = symbols.get(&self.func.id).expect(format!("looking up function {}", self.func.id).as_str());
        println!("symbol: {:?}", symbol);
        let args = self.args[0].clone().to_rust(ctx, options, symbols).expect(format!("parsing arguments {:?}", self.args[0]).as_str());
        Ok(quote!(#name(#args)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_of_function() {
        let options = PythonOptions::default();
        let result = crate::parse("def foo(a = 7):
    pass

foo(b=9)", "test").unwrap();
        println!("Python tree: {:#?}", result);
        let code = result.to_rust(CodeGenContext::Module("test".to_string()), options, SymbolTableScopes::new()).unwrap();
        println!("Rust code: {}", code);
    }
}