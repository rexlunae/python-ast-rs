use proc_macro2::TokenStream;
use pyo3::{Bound, PyAny, FromPyObject, PyResult, prelude::PyAnyMethods, types::PyTypeMethods};
use quote::{format_ident, quote};

use crate::{dump, CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
//#[pyo3(transparent)]
pub struct Attribute {
    value: Box<ExprType>,
    attr: String,
    ctx: String,
}

impl<'a> FromPyObject<'a> for Attribute {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let value = ob.getattr("value").expect("Attribute.value");
        let attr = ob.getattr("attr").expect("Attribute.attr");
        let ctx = ob
            .getattr("ctx")
            .expect("getting attribute context")
            .get_type()
            .name()
            .expect(
                ob.error_message(
                    "<unknown>",
                    format!("extracting type name {:?} in attribute", dump(ob, None)),
                )
                .as_str(),
            );
        Ok(Attribute {
            value: Box::new(value.extract().expect("Attribute.value")),
            attr: attr.extract().expect("Attribute.attr"),
            ctx: ctx.to_string(),
        })
    }
}

impl<'a> CodeGen for Attribute {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let value_tokens = self.value.to_rust(ctx, options, symbols)?;
        let value_str = value_tokens.to_string();
        let attr = format_ident!("{}", self.attr);
        
        // Determine if this is a module access or a field/method access
        // Module names are typically lowercase and match Python stdlib modules
        let is_module_access = matches!(value_str.as_str(), 
            "sys" | "os" | "subprocess" | "json" | "urllib" | "xml" | "asyncio" |
            "os :: path" | "os::path" // for nested modules
        );
        
        if is_module_access {
            // Use :: for module access (Python's sys.executable becomes sys::executable)
            // Special handling for LazyLock static variables that need dereferencing
            let needs_deref = matches!((value_str.as_str(), self.attr.as_str()), 
                ("sys", "executable") | ("sys", "argv") | ("os", "environ")
            );
            
            if needs_deref {
                // Wrap dereferenced values in parentheses to ensure correct precedence
                // This prevents *sys::executable.to_string() and ensures (*sys::executable).to_string()
                Ok(quote!((*#value_tokens::#attr)))
            } else {
                Ok(quote!(#value_tokens::#attr))
            }
        } else {
            // Use . for field/method access (Python's obj.field becomes obj.field)
            Ok(quote!(#value_tokens.#attr))
        }
    }
}
