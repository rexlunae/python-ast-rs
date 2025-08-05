use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{CodeGen, CodeGenContext, ExprType, Keyword, PythonOptions, SymbolTableScopes, extract_required_attr};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Call {
    pub func: Box<ExprType>,
    pub args: Vec<ExprType>,
    pub keywords: Vec<Keyword>,
}

impl<'a> FromPyObject<'a> for Call {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let func: ExprType = extract_required_attr(ob, "func", "function call expression")?;
        let args: Vec<ExprType> = extract_required_attr(ob, "args", "function call arguments")?;
        let keywords: Vec<Keyword> = extract_required_attr(ob, "keywords", "function call keywords")?;
        
        Ok(Call {
            func: Box::new(func),
            args,
            keywords,
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
        
        // Check if we're in an async context and if the function being called is async
        let call_expr = quote!(#name(#(#all_args),*));
        
        match ctx {
            CodeGenContext::Async(_) => {
                // In async context, we assume Python async functions need .await
                // We'll check if the function name suggests it's async
                let name_str = format!("{}", name);
                if name_str.contains("async") || 
                   name_str.starts_with("a") || // Common async function naming
                   // TODO: Better async function detection based on symbol table
                   false {
                    Ok(quote!(#call_expr.await))
                } else {
                    // For now, just return the regular call
                    // In a full implementation, we'd track which functions are async
                    Ok(call_expr)
                }
            },
            _ => Ok(call_expr)
        }
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
        let _code = result
            .to_rust(
                CodeGenContext::Module("test".to_string()),
                options,
                SymbolTableScopes::new(),
            )
            .unwrap();
    }
}
