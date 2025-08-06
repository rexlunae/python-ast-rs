use log::debug;
use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use crate::ast::tree::statement::PyStatementTrait;

use crate::{
    CodeGen, CodeGenContext, ExprType, Object, ParameterList, PythonOptions, Statement,
    StatementType, SymbolTableNode, SymbolTableScopes,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub args: ParameterList,
    pub body: Vec<Statement>,
    pub decorator_list: Vec<ExprType>,
}

impl<'a> FromPyObject<'a> for FunctionDef {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let name: String = ob.getattr("name")?.extract()?;
        let args: ParameterList = ob.getattr("args")?.extract()?;
        let body: Vec<Statement> = ob.getattr("body")?.extract()?;
        
        // Extract decorator_list as Vec<ExprType>
        let decorator_list: Vec<ExprType> = ob.getattr("decorator_list")?.extract().unwrap_or_default();
        
        Ok(FunctionDef {
            name,
            args,
            body,
            decorator_list,
        })
    }
}

impl PyStatementTrait for FunctionDef {
}

impl CodeGen for FunctionDef {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        symbols.insert(
            self.name.clone(),
            SymbolTableNode::FunctionDef(self.clone()),
        );
        symbols
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: SymbolTableScopes,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut streams = TokenStream::new();
        let fn_name = format_ident!("{}", self.name);

        // The Python convention is that functions that begin with a single underscore,
        // it's private. Otherwise, it's public. We formalize that by default.
        let visibility = if self.name.starts_with("_") && !self.name.starts_with("__") {
            quote!()  // private, no visibility modifier
        } else if self.name.starts_with("__") && self.name.ends_with("__") {
            quote!(pub(crate))  // dunder methods are crate-visible
        } else {
            quote!(pub)  // regular methods are public
        };

        let is_async = match ctx.clone() {
            CodeGenContext::Async(_) => {
                quote!(async)
            }
            _ => quote!(),
        };

        let parameters = self
            .args
            .clone()
            .to_rust(ctx.clone(), options.clone(), symbols.clone())
            .expect(format!("parsing arguments {:?}", self.args).as_str());

        for s in self.body.iter() {
            streams.extend(
                s.clone()
                    .to_rust(ctx.clone(), options.clone(), symbols.clone())
                    .expect(format!("parsing statement {:?}", s).as_str()),
            );
            streams.extend(quote!(;));
        }

        let function = if let Some(docstring) = self.get_docstring() {
            // Convert docstring to Rust doc comments
            let doc_lines: Vec<_> = docstring
                .lines()
                .map(|line| {
                    if line.trim().is_empty() {
                        quote! { #[doc = ""] }
                    } else {
                        let doc_line = format!("{}", line);
                        quote! { #[doc = #doc_line] }
                    }
                })
                .collect();
            
            quote! {
                #(#doc_lines)*
                #visibility #is_async fn #fn_name(#parameters) {
                    #streams
                }
            }
        } else {
            quote! {
                #visibility #is_async fn #fn_name(#parameters) {
                    #streams
                }
            }
        };

        debug!("function: {}", function);
        Ok(function)
    }
}

impl FunctionDef {
    fn get_docstring(&self) -> Option<String> {
        if self.body.is_empty() {
            return None;
        }
        
        let expr = self.body[0].clone();
        match expr.statement {
            StatementType::Expr(e) => match e.value {
                ExprType::Constant(c) => {
                    let raw_string = c.to_string();
                    // Clean up the docstring for Rust documentation
                    Some(self.format_docstring(&raw_string))
                },
                _ => None,
            },
            _ => None,
        }
    }
    
    fn format_docstring(&self, raw: &str) -> String {
        // Remove surrounding quotes
        let content = raw.trim_matches('"');
        
        // Split into lines and clean up Python-style indentation
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return String::new();
        }
        
        // First line is usually the summary
        let mut formatted = vec![lines[0].trim().to_string()];
        
        if lines.len() > 1 {
            // Add empty line after summary if there are more lines
            if !lines[0].trim().is_empty() && !lines[1].trim().is_empty() {
                formatted.push(String::new());
            }
            
            // Process remaining lines, cleaning up indentation
            for line in lines.iter().skip(1) {
                let cleaned = line.trim();
                if cleaned.starts_with("Args:") {
                    formatted.push(String::new());
                    formatted.push("# Arguments".to_string());
                } else if cleaned.starts_with("Returns:") {
                    formatted.push(String::new());
                    formatted.push("# Returns".to_string());
                } else if cleaned.starts_with("Example:") {
                    formatted.push(String::new());
                    formatted.push("# Examples".to_string());
                } else if cleaned.starts_with(">>>") {
                    // Convert Python examples to Rust doc test format
                    formatted.push(format!("```rust"));
                    formatted.push(format!("// {}", cleaned));
                } else if !cleaned.is_empty() {
                    formatted.push(cleaned.to_string());
                }
            }
            
            // Close any open code blocks
            if content.contains(">>>") {
                formatted.push("```".to_string());
            }
        }
        
        formatted.join("\n")
    }
}

impl Object for FunctionDef {}
