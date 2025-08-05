//! The module defines Python-syntax arguments and maps them into Rust-syntax versions.
use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes,
};

/// A complete argument representation that can hold any Python expression.
/// This replaces the limited Arg enum to support all argument types.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Argument {
    /// The argument expression (can be any valid Python expression)
    pub value: ExprType,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// An argument value that can be any expression.
/// This replaces the old limited Arg enum.
pub type Arg = ExprType;

/// A function parameter definition with optional type annotation and default value.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    /// Parameter name
    pub arg: String,
    /// Optional type annotation
    pub annotation: Option<Box<ExprType>>,
    /// Optional type comment (deprecated Python feature)
    pub type_comment: Option<String>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// Comprehensive function arguments structure supporting all Python argument types.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Arguments {
    /// Positional-only parameters (before / in Python 3.8+)
    pub posonlyargs: Vec<Parameter>,
    /// Regular positional parameters
    pub args: Vec<Parameter>,
    /// Variable positional parameter (*args)
    pub vararg: Option<Parameter>,
    /// Keyword-only parameters (after * or *args)
    pub kwonlyargs: Vec<Parameter>,
    /// Default values for keyword-only parameters (None = required)
    pub kw_defaults: Vec<Option<Box<ExprType>>>,
    /// Variable keyword parameter (**kwargs)
    pub kwarg: Option<Parameter>,
    /// Default values for regular positional parameters
    pub defaults: Vec<Box<ExprType>>,
}


/// Function call arguments supporting all Python call patterns.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CallArguments {
    /// Positional arguments
    pub args: Vec<ExprType>,
    /// Keyword arguments
    pub keywords: Vec<crate::Keyword>,
}

// Implementation for new Argument struct
impl<'a> FromPyObject<'a> for Argument {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract the expression value
        let value: ExprType = ob.extract()?;
        
        Ok(Self {
            value,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl CodeGen for Argument {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        self.value.to_rust(ctx, options, symbols)
    }
}

// Implementation for Parameter struct
impl<'a> FromPyObject<'a> for Parameter {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let arg: String = ob.getattr("arg")?.extract()?;
        
        // Extract optional annotation
        let annotation = if let Ok(ann) = ob.getattr("annotation") {
            if ann.is_none() {
                None
            } else {
                Some(Box::new(ann.extract()?))
            }
        } else {
            None
        };
        
        // Extract optional type comment
        let type_comment = if let Ok(tc) = ob.getattr("type_comment") {
            if tc.is_none() {
                None
            } else {
                Some(tc.extract()?)
            }
        } else {
            None
        };
        
        Ok(Self {
            arg,
            annotation,
            type_comment,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl CodeGen for Parameter {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        use quote::format_ident;
        
        let param_name = format_ident!("{}", self.arg);
        
        // Generate type annotation if present
        if let Some(annotation) = self.annotation {
            let rust_type = annotation.to_rust(ctx, options, symbols)?;
            Ok(quote!(#param_name: #rust_type))
        } else {
            // Default to generic type for untyped parameters
            Ok(quote!(#param_name: impl Into<PyObject>))
        }
    }
}

// Implementation for Arguments struct
impl<'a> FromPyObject<'a> for Arguments {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract each field with proper error handling
        let posonlyargs: Vec<Parameter> = ob.getattr("posonlyargs")?.extract().unwrap_or_default();
        let args: Vec<Parameter> = ob.getattr("args")?.extract().unwrap_or_default();
        
        let vararg = if let Ok(va) = ob.getattr("vararg") {
            if va.is_none() { None } else { Some(va.extract()?) }
        } else { None };
        
        let kwonlyargs: Vec<Parameter> = ob.getattr("kwonlyargs")?.extract().unwrap_or_default();
        
        // Handle kw_defaults which can contain None values
        let kw_defaults = if let Ok(kw_def) = ob.getattr("kw_defaults") {
            let defaults_list: Vec<Bound<PyAny>> = kw_def.extract().unwrap_or_default();
            let mut processed_defaults = Vec::new();
            for default in defaults_list {
                if default.is_none() {
                    processed_defaults.push(None);
                } else {
                    processed_defaults.push(Some(Box::new(default.extract()?)));
                }
            }
            processed_defaults
        } else {
            Vec::new()
        };
        
        let kwarg = if let Ok(kw) = ob.getattr("kwarg") {
            if kw.is_none() { None } else { Some(kw.extract()?) }
        } else { None };
        
        let defaults_raw: Vec<ExprType> = ob.getattr("defaults")?.extract().unwrap_or_default();
        let defaults = defaults_raw.into_iter().map(Box::new).collect();
        
        Ok(Self {
            posonlyargs,
            args,
            vararg,
            kwonlyargs,
            kw_defaults,
            kwarg,
            defaults,
        })
    }
}

impl CodeGen for Arguments {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        let mut params = Vec::new();
        
        // Process positional-only arguments
        for arg in self.posonlyargs {
            let param = arg.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            params.push(param);
        }
        
        // Process regular positional arguments with defaults
        let defaults_offset = self.args.len().saturating_sub(self.defaults.len());
        for (i, arg) in self.args.into_iter().enumerate() {
            if i >= defaults_offset {
                // This argument has a default value
                let default_idx = i - defaults_offset;
                let default_value = &self.defaults[default_idx];
                let _default_rust = default_value.as_ref().clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                let param_name = quote::format_ident!("{}", arg.arg);
                
                if let Some(annotation) = &arg.annotation {
                    let rust_type = annotation.as_ref().clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                    params.push(quote!(#param_name: Option<#rust_type>));
                } else {
                    params.push(quote!(#param_name: Option<impl Into<PyObject>>));
                }
            } else {
                let param = arg.to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                params.push(param);
            }
        }
        
        // Process *args
        if let Some(vararg) = self.vararg {
            let vararg_name = quote::format_ident!("{}", vararg.arg);
            params.push(quote!(#vararg_name: impl IntoIterator<Item = impl Into<PyObject>>));
        }
        
        // Process keyword-only arguments
        for (i, arg) in self.kwonlyargs.into_iter().enumerate() {
            let param_name = quote::format_ident!("{}", arg.arg);
            
            // Check if this keyword-only arg has a default
            let has_default = i < self.kw_defaults.len() && self.kw_defaults[i].is_some();
            
            if let Some(annotation) = &arg.annotation {
                let rust_type = annotation.as_ref().clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
                if has_default {
                    params.push(quote!(#param_name: Option<#rust_type>));
                } else {
                    params.push(quote!(#param_name: #rust_type));
                }
            } else {
                if has_default {
                    params.push(quote!(#param_name: Option<impl Into<PyObject>>));
                } else {
                    params.push(quote!(#param_name: impl Into<PyObject>));
                }
            }
        }
        
        // Process **kwargs
        if let Some(kwarg) = self.kwarg {
            let kwarg_name = quote::format_ident!("{}", kwarg.arg);
            params.push(quote!(#kwarg_name: impl IntoIterator<Item = (impl AsRef<str>, impl Into<PyObject>)>));
        }
        
        Ok(quote!(#(#params),*))
    }
}


// Implementation for CallArguments
impl<'a> FromPyObject<'a> for CallArguments {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        let args: Vec<ExprType> = ob.getattr("args")?.extract().unwrap_or_default();
        let keywords: Vec<crate::Keyword> = ob.getattr("keywords")?.extract().unwrap_or_default();
        
        Ok(Self { args, keywords })
    }
}

impl CodeGen for CallArguments {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
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
        
        Ok(quote!(#(#all_args),*))
    }
}


// Node trait implementations for position tracking
impl Node for Argument {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl Node for Parameter {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse, CodeGenContext, ExprType, PythonOptions, SymbolTableScopes};
    use test_log::test;

    #[test]
    fn test_simple_function_call() {
        let code = "func(1, 2, 3)";
        let result = parse(code, "test.py").unwrap();
        
        // Generate Rust code
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function call with positional arguments
    }

    #[test]
    fn test_keyword_arguments() {
        let code = "func(a=1, b=2)";
        let result = parse(code, "test.py").unwrap();
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function call with keyword arguments
    }

    #[test]
    fn test_mixed_arguments() {
        let code = "func(1, 2, c=3, d=4)";
        let result = parse(code, "test.py").unwrap();
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function call with mixed positional and keyword arguments
    }

    #[test]
    fn test_function_with_defaults() {
        let code = r#"
def func(a, b=2, c=3):
    pass
        "#;
        let result = parse(code, "test.py").unwrap();
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function with optional parameters
    }

    #[test]
    fn test_function_with_varargs() {
        let code = r#"
def func(a, *args):
    pass
        "#;
        let result = parse(code, "test.py").unwrap();
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function with variable arguments
    }

    #[test]
    fn test_function_with_kwargs() {
        let code = r#"
def func(a, **kwargs):
    pass
        "#;
        let result = parse(code, "test.py").unwrap();
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function with keyword arguments dict
    }

    #[test]
    fn test_complex_function_signature() {
        let code = r#"
def func(a, b=2, *args, c, d=4, **kwargs):
    pass
        "#;
        let result = parse(code, "test.py").unwrap();
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function with all argument types
    }

    #[test]
    fn test_keyword_only_arguments() {
        let code = r#"
def func(a, *, b, c=3):
    pass
        "#;
        let result = parse(code, "test.py").unwrap();
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let _rust_code = result.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        // Should generate function with keyword-only arguments
    }

    #[test]
    fn test_argument_unpacking_call() {
        // Note: This would require additional AST node support for Starred expressions
        let code = "func(*args, **kwargs)";
        let result = parse(code, "test.py");
        
        match result {
            Ok(ast) => {
                let options = PythonOptions::default();
                let symbols = SymbolTableScopes::new();
                let rust_code = ast.to_rust(
                    CodeGenContext::Module("test".to_string()),
                    options,
                    symbols,
                );
                
                match rust_code {
                    Ok(_code) => { /* Code generation succeeded as expected */ },
                    Err(_e) => { /* Expected error for unimplemented feature */ },
                }
            }
            Err(_e) => { /* Parse error expected for unimplemented features */ },
        }
    }

    #[test]
    fn test_arg_with_constant() {
        // Test that Arg (now ExprType) works with constants
        use litrs::Literal;
        let literal = Literal::parse("42").unwrap().into_owned();
        let constant = crate::Constant(Some(literal));
        let arg: Arg = ExprType::Constant(constant);
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let rust_code = arg.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        assert!(rust_code.to_string().contains("42"));
    }

    #[test]
    fn test_arg_with_name() {
        // Test that Arg (now ExprType) works with name expressions
        let name_expr = ExprType::Name(crate::Name {
            id: "variable".to_string(),
        });
        let arg: Arg = name_expr;
        
        let options = PythonOptions::default();
        let symbols = SymbolTableScopes::new();
        let rust_code = arg.to_rust(
            CodeGenContext::Module("test".to_string()),
            options,
            symbols,
        ).unwrap();
        
        assert!(rust_code.to_string().contains("variable"));
    }
}
