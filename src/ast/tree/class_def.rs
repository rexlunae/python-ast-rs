//! A lot of languages, Python included, have a concept of a class, which combines the definition of a data type with
//! an interface. In dynamic languages like Python, the class itself is a memory object, that can be permutated at runtime,
//! however, this is probably usually a bad idea. Classes can contain:
//! 1. Methods (special functions)
//! 2. properties (attributes of the data element)
//! 3. Base classes (for inheritace)
//! 4. static data
//! 5. Additional classes
//!
//! There is one construct in Rust that can emcompass all of these things: a module. So, we use modules to model classes
//! following these rules:
//! 1. The module is given the name of the class. Contrary to other Rust modules, this is typically SnakeCase.
//! 2. The main data type defined by the class is a struct inside the module, and called Data.
//! 3. The Data struct can take two forms:
//!   a. If the properties of the class can be fully inferred, Data will be a simple struct and the attributes will be defined as fields of the struct.
//!   b. If the properties of the class cannot be fully inferred, such as if the class is accessed as a dictionary, Data will be a HashMap<String, _>,
//!   and the values will be accessed through it.
//! 4. Static data will be declared with lazy_static inside the module.
//! 5. Additional classes will be nested inside the module, and therefore they appear as modules inside a module.
//! 6. Each class also contains a trait, named Cls, which is used in inheritance.
//! 7. Each method of the class in Python will be translated to have a prototype in Cls. If it is possible to implement the method as a default method,
//! it will be, otherwise (if the method refers to attributes of the class), a prototype will be added to Cls, and the implementation will be done inside
//! an impl Cls for Data block.
//! 8. Cls will implement Clone, Default.

use proc_macro2::TokenStream;
use pyo3::FromPyObject;
use quote::{format_ident, quote};

use crate::{
    CodeGen, CodeGenContext, ExprType, Name, PythonOptions, Statement, StatementType,
    SymbolTableNode, SymbolTableScopes,
};

use log::debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, FromPyObject, Serialize, Deserialize, PartialEq)]
pub struct ClassDef {
    pub name: String,
    pub bases: Vec<Name>,
    pub keywords: Vec<String>,
    pub body: Vec<Statement>,
}

impl CodeGen for ClassDef {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        let mut symbols = symbols;
        symbols.insert(self.name.clone(), SymbolTableNode::ClassDef(self.clone()));
        symbols
    }

    fn to_rust(
        self,
        _ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut streams = TokenStream::new();
        let class_name = format_ident!("{}", self.name);

        // The Python convention is that functions that begin with a single underscore,
        // it's private. Otherwise, it's public. We formalize that by default.
        let visibility = if self.name.starts_with("_") && !self.name.starts_with("__") {
            format_ident!("")
        } else if self.name.starts_with("__") && self.name.ends_with("__") {
            format_ident!("pub(crate)")
        } else {
            format_ident!("pub")
        };

        // bases will be empty if there are no base classes, which prevents any base traits
        // being added, and also prevents the : from being emitted.
        let mut bases = TokenStream::new();
        if self.bases.len() > 0 {
            bases.extend(quote!(:));
            let base_name = format_ident!("{}", self.bases[0].id);
            bases.extend(quote!(#base_name::Cls));
            for base in &self.bases[1..] {
                bases.extend(quote!(+));
                let base_name = format_ident!("{}", base.id);
                bases.extend(quote!(#base_name));
            }
        }

        for s in self.body.clone() {
            streams.extend(
                s.clone()
                    .to_rust(CodeGenContext::Class, options.clone(), symbols.clone())
                    .expect(format!("Failed to parse statement {:?}", s).as_str()),
            );
        }

        let class = if let Some(docstring) = self.get_docstring() {
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
                #visibility mod #class_name {
                    use super::*;
                    #visibility trait Cls #bases {
                        #streams
                    }
                    #[derive(Clone, Default)]
                    #visibility struct Data {

                    }
                    impl Cls for Data {}
                }
            }
        } else {
            quote! {
                #visibility mod #class_name {
                    use super::*;
                    #visibility trait Cls #bases {
                        #streams
                    }
                    #[derive(Clone, Default)]
                    #visibility struct Data {

                    }
                    impl Cls for Data {}
                }
            }
        };
        debug!("class: {}", class);
        Ok(class)
    }
}

impl ClassDef {
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
                if !cleaned.is_empty() {
                    formatted.push(cleaned.to_string());
                }
            }
        }
        
        formatted.join("\n")
    }
}
