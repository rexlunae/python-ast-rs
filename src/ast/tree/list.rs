use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, types::PyAnyMethods};
use quote::quote;

use crate::{dump, CodeGen, CodeGenContext, PythonOptions, SymbolTableScopes};

// There are two concepts of List in the same place here. There's the "List" type that represents a node from the Python AST,
// as received by the Rust AST converter, and there's the List representation of the Python List type. For the sake of
// consistency, we're using the same type as we use to model
pub type ListContents = crate::pytypes::List<dyn CodeGen>;

#[derive(Clone, Default)]
pub struct List<'a> {
    pub elts: Vec<Bound<'a, PyAny>>,
    pub ctx: Option<String>,
}

impl std::fmt::Debug for List<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("elts", &format!("Vec<Bound<PyAny>> (len: {})", self.elts.len()))
            .field("ctx", &self.ctx)
            .finish()
    }
}

impl<'a> FromPyObject<'a> for List<'a> {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> pyo3::PyResult<Self> {
        let elts: Vec<Bound<'a, PyAny>> = ob.getattr("elts")?.extract()?;
        let ctx: Option<String> = ob.getattr("ctx").ok().and_then(|v| v.extract().ok());
        Ok(List { elts, ctx })
    }
}

impl<'a> CodeGen for List<'a> {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        use crate::ExprType;
        
        let mut elements = Vec::new();
        let mut has_starred = false;
        
        log::debug!("================Processing list with {} elements", self.elts.len());
        for elt in self.elts {
            log::debug!("elt: {}", dump(&elt, None)?);
            
            // Extract the element as ExprType and convert to Rust
            let expr: ExprType = elt.extract()?;
            
            // Check if this is a starred expression
            match &expr {
                ExprType::Starred(_) => {
                    has_starred = true;
                    let rust_code = expr.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                    let rust_str = rust_code.to_string();
                    
                    // If it's a starred sys::argv, we need special handling
                    if rust_str.contains("sys :: argv") {
                        // Instead of adding individual elements, we'll build the vector differently
                        elements.push(quote! { /* STARRED_ARGV */ });
                    } else {
                        elements.push(rust_code);
                    }
                }
                _ => {
                    let rust_code = expr.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                    elements.push(rust_code);
                }
            }
        }
        
        // If we have starred expressions, especially sys::argv, handle it specially
        if has_starred && elements.iter().any(|e| e.to_string().contains("STARRED_ARGV")) {
            // Create a special vector construction that handles sys::argv unpacking
            let mut final_elements = Vec::new();
            for element in elements {
                let elem_str = element.to_string();
                if elem_str.contains("STARRED_ARGV") {
                    // Skip the placeholder and add the argv unpacking
                    continue;
                } else {
                    final_elements.push(element);
                }
            }
            
            // Build the vector with sys::argv unpacking
            Ok(quote! {
                {
                    let mut vec = Vec::new();
                    #(vec.push(#final_elements);)*
                    vec.extend((*sys::argv).iter().cloned());
                    vec
                }
            })
        } else {
            Ok(quote!(/* LIST_GENERATED */ vec![#(#elements),*]))
        }
    }
}

// It's fairly easy to break the automatic parsing of parameter structs, so we need to have fairly sophisticated
// test coverage for the various types of
#[cfg(test)]
mod tests {
    use crate::ExprType;
    use crate::StatementType;
    use std::panic;
    use test_log::test;

    #[test]
    fn parse_list() {
        let module = crate::parse("[1, 2, 3]", "nothing.py").unwrap();
        let statement = module.raw.body[0].statement.clone();
        match statement {
            StatementType::Expr(e) => match e.value {
                ExprType::List(list) => {
                    log::debug!("{:#?}", list);
                    assert_eq!(list.len(), 3);
                }
                _ => panic!("Could not find inner expression"),
            },
            _ => panic!("Could not find outer expression."),
        }
    }
}
