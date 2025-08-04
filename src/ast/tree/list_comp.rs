use proc_macro2::TokenStream;
use pyo3::{Bound, FromPyObject, PyAny, PyResult, prelude::PyAnyMethods};
use quote::quote;
use serde::{Deserialize, Serialize};

use crate::{
    CodeGen, CodeGenContext, ExprType, Node, PythonOptions, SymbolTableScopes,
    PyAttributeExtractor, extract_list,
};

/// List comprehension (e.g., [x ** 2 for x in range(10) if x % 2 == 0])
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ListComp {
    /// The element expression being computed
    pub elt: Box<ExprType>,
    /// The generators (for clauses)
    pub generators: Vec<Comprehension>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// Set comprehension (e.g., {x for x in range(10) if x % 2 == 0})
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SetComp {
    /// The element expression being computed
    pub elt: Box<ExprType>,
    /// The generators (for clauses)
    pub generators: Vec<Comprehension>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// Generator expression (e.g., (x for x in range(10) if x % 2 == 0))
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GeneratorExp {
    /// The element expression being computed
    pub elt: Box<ExprType>,
    /// The generators (for clauses)
    pub generators: Vec<Comprehension>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// Dictionary comprehension (e.g., {k: v for k, v in items.items()})
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DictComp {
    /// The key expression being computed
    pub key: Box<ExprType>,
    /// The value expression being computed
    pub value: Box<ExprType>,
    /// The generators (for clauses)
    pub generators: Vec<Comprehension>,
    /// Position information
    pub lineno: Option<usize>,
    pub col_offset: Option<usize>,
    pub end_lineno: Option<usize>,
    pub end_col_offset: Option<usize>,
}

/// A comprehension generator (for x in iter if condition)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Comprehension {
    /// The target variable(s) (e.g., x in "for x in range(10)")
    pub target: ExprType,
    /// The iterable expression (e.g., range(10) in "for x in range(10)")
    pub iter: ExprType,
    /// The conditions (if clauses)
    pub ifs: Vec<ExprType>,
    /// Whether this is an async comprehension
    pub is_async: bool,
}

impl<'a> FromPyObject<'a> for ListComp {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract the element expression
        let elt = ob.extract_attr_with_context("elt", "list comprehension element")?;
        let elt: ExprType = elt.extract()?;
        
        // Extract generators
        let generators: Vec<Comprehension> = extract_list(ob, "generators", "list comprehension generators")?;
        
        Ok(ListComp {
            elt: Box::new(elt),
            generators,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl<'a> FromPyObject<'a> for SetComp {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract the element expression
        let elt = ob.extract_attr_with_context("elt", "set comprehension element")?;
        let elt: ExprType = elt.extract()?;
        
        // Extract generators
        let generators: Vec<Comprehension> = extract_list(ob, "generators", "set comprehension generators")?;
        
        Ok(SetComp {
            elt: Box::new(elt),
            generators,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl<'a> FromPyObject<'a> for GeneratorExp {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract the element expression
        let elt = ob.extract_attr_with_context("elt", "generator expression element")?;
        let elt: ExprType = elt.extract()?;
        
        // Extract generators
        let generators: Vec<Comprehension> = extract_list(ob, "generators", "generator expression generators")?;
        
        Ok(GeneratorExp {
            elt: Box::new(elt),
            generators,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl<'a> FromPyObject<'a> for DictComp {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract the key expression
        let key = ob.extract_attr_with_context("key", "dict comprehension key")?;
        let key: ExprType = key.extract()?;
        
        // Extract the value expression
        let value = ob.extract_attr_with_context("value", "dict comprehension value")?;
        let value: ExprType = value.extract()?;
        
        // Extract generators
        let generators: Vec<Comprehension> = extract_list(ob, "generators", "dict comprehension generators")?;
        
        Ok(DictComp {
            key: Box::new(key),
            value: Box::new(value),
            generators,
            lineno: ob.lineno(),
            col_offset: ob.col_offset(),
            end_lineno: ob.end_lineno(),
            end_col_offset: ob.end_col_offset(),
        })
    }
}

impl<'a> FromPyObject<'a> for Comprehension {
    fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
        // Extract target
        let target = ob.extract_attr_with_context("target", "comprehension target")?;
        let target: ExprType = target.extract()?;
        
        // Extract iter
        let iter = ob.extract_attr_with_context("iter", "comprehension iter")?;
        let iter: ExprType = iter.extract()?;
        
        // Extract ifs (list of conditions)
        let ifs: Vec<ExprType> = extract_list(ob, "ifs", "comprehension conditions").unwrap_or_default();
        
        // Extract is_async
        let is_async: bool = ob.getattr("is_async")?.extract().unwrap_or(false);
        
        Ok(Comprehension {
            target,
            iter,
            ifs,
            is_async,
        })
    }
}

impl Node for ListComp {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl Node for SetComp {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl Node for GeneratorExp {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl Node for DictComp {
    fn lineno(&self) -> Option<usize> { self.lineno }
    fn col_offset(&self) -> Option<usize> { self.col_offset }
    fn end_lineno(&self) -> Option<usize> { self.end_lineno }
    fn end_col_offset(&self) -> Option<usize> { self.end_col_offset }
}

impl CodeGen for ListComp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        // Process the element and generators
        let symbols = (*self.elt).clone().find_symbols(symbols);
        self.generators.into_iter().fold(symbols, |acc, generator| {
            let acc = generator.target.find_symbols(acc);
            let acc = generator.iter.find_symbols(acc);
            generator.ifs.into_iter().fold(acc, |acc, if_expr| if_expr.find_symbols(acc))
        })
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // For now, generate a simple Vec collection since Rust doesn't have list comprehensions
        // This is a simplified translation that doesn't handle all cases
        if self.generators.len() == 1 {
            let generator = &self.generators[0];
            let elt = (*self.elt).clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let iter_expr = generator.iter.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            
            if generator.ifs.is_empty() {
                // Simple case: [expr for x in iter] -> iter.map(|x| expr).collect()
                Ok(quote! {
                    (#iter_expr).into_iter().map(|_item| #elt).collect::<Vec<_>>()
                })
            } else {
                // With conditions: [expr for x in iter if cond] -> iter.filter(cond).map(expr).collect()
                let conditions: Result<Vec<_>, _> = generator.ifs.iter()
                    .map(|if_expr| if_expr.clone().to_rust(ctx.clone(), options.clone(), symbols.clone()))
                    .collect();
                let conditions = conditions?;
                Ok(quote! {
                    (#iter_expr).into_iter()
                        .filter(|_item| { #(#conditions)&&* })
                        .map(|_item| #elt)
                        .collect::<Vec<_>>()
                })
            }
        } else {
            // Multiple generators would need nested iteration - this is complex
            // For now, return a placeholder
            Ok(quote! {
                vec![] // Complex list comprehension with multiple generators not fully supported
            })
        }
    }
}

impl CodeGen for SetComp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        // Process the element and generators
        let symbols = (*self.elt).clone().find_symbols(symbols);
        self.generators.into_iter().fold(symbols, |acc, generator| {
            let acc = generator.target.find_symbols(acc);
            let acc = generator.iter.find_symbols(acc);
            generator.ifs.into_iter().fold(acc, |acc, if_expr| if_expr.find_symbols(acc))
        })
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // For now, generate a simple HashSet collection since Rust doesn't have set comprehensions
        // This is a simplified translation that doesn't handle all cases
        if self.generators.len() == 1 {
            let generator = &self.generators[0];
            let elt = (*self.elt).clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let iter_expr = generator.iter.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            
            if generator.ifs.is_empty() {
                // Simple case: {expr for x in iter} -> iter.map(|x| expr).collect()
                Ok(quote! {
                    (#iter_expr).into_iter().map(|_item| #elt).collect::<std::collections::HashSet<_>>()
                })
            } else {
                // With conditions: {expr for x in iter if cond} -> iter.filter(cond).map(expr).collect()
                let conditions: Result<Vec<_>, _> = generator.ifs.iter()
                    .map(|if_expr| if_expr.clone().to_rust(ctx.clone(), options.clone(), symbols.clone()))
                    .collect();
                let conditions = conditions?;
                Ok(quote! {
                    (#iter_expr).into_iter()
                        .filter(|_item| { #(#conditions)&&* })
                        .map(|_item| #elt)
                        .collect::<std::collections::HashSet<_>>()
                })
            }
        } else {
            // Multiple generators would need nested iteration - this is complex
            // For now, return a placeholder
            Ok(quote! {
                std::collections::HashSet::new() // Complex set comprehension with multiple generators not fully supported
            })
        }
    }
}

impl CodeGen for GeneratorExp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        // Process the element and generators
        let symbols = (*self.elt).clone().find_symbols(symbols);
        self.generators.into_iter().fold(symbols, |acc, generator| {
            let acc = generator.target.find_symbols(acc);
            let acc = generator.iter.find_symbols(acc);
            generator.ifs.into_iter().fold(acc, |acc, if_expr| if_expr.find_symbols(acc))
        })
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // For now, generate a simple iterator since Rust doesn't have generator expressions
        // This is a simplified translation that doesn't handle all cases
        if self.generators.len() == 1 {
            let generator = &self.generators[0];
            let elt = (*self.elt).clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let iter_expr = generator.iter.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            
            if generator.ifs.is_empty() {
                // Simple case: (expr for x in iter) -> iter.map(|x| expr)
                Ok(quote! {
                    (#iter_expr).into_iter().map(|_item| #elt)
                })
            } else {
                // With conditions: (expr for x in iter if cond) -> iter.filter(cond).map(expr)
                let conditions: Result<Vec<_>, _> = generator.ifs.iter()
                    .map(|if_expr| if_expr.clone().to_rust(ctx.clone(), options.clone(), symbols.clone()))
                    .collect();
                let conditions = conditions?;
                Ok(quote! {
                    (#iter_expr).into_iter()
                        .filter(|_item| { #(#conditions)&&* })
                        .map(|_item| #elt)
                })
            }
        } else {
            // Multiple generators would need nested iteration - this is complex
            // For now, return a placeholder
            Ok(quote! {
                std::iter::empty() // Complex generator expression with multiple generators not fully supported
            })
        }
    }
}

impl CodeGen for DictComp {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn find_symbols(self, symbols: Self::SymbolTable) -> Self::SymbolTable {
        // Process the key, value and generators
        let symbols = (*self.key).clone().find_symbols(symbols);
        let symbols = (*self.value).clone().find_symbols(symbols);
        self.generators.into_iter().fold(symbols, |acc, generator| {
            let acc = generator.target.find_symbols(acc);
            let acc = generator.iter.find_symbols(acc);
            generator.ifs.into_iter().fold(acc, |acc, if_expr| if_expr.find_symbols(acc))
        })
    }

    fn to_rust(
        self,
        ctx: Self::Context,
        options: Self::Options,
        symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // For now, generate a simple HashMap collection since Rust doesn't have dict comprehensions
        if self.generators.len() == 1 {
            let generator = &self.generators[0];
            let key = (*self.key).clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let value = (*self.value).clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let iter_expr = generator.iter.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            
            if generator.ifs.is_empty() {
                // Simple case: {k: v for x in iter} -> iter.map(|x| (k, v)).collect()
                Ok(quote! {
                    (#iter_expr).into_iter().map(|_item| (#key, #value)).collect::<std::collections::HashMap<_, _>>()
                })
            } else {
                // With conditions: {k: v for x in iter if cond}
                let conditions: Result<Vec<_>, _> = generator.ifs.iter()
                    .map(|if_expr| if_expr.clone().to_rust(ctx.clone(), options.clone(), symbols.clone()))
                    .collect();
                let conditions = conditions?;
                Ok(quote! {
                    (#iter_expr).into_iter()
                        .filter(|_item| { #(#conditions)&&* })
                        .map(|_item| (#key, #value))
                        .collect::<std::collections::HashMap<_, _>>()
                })
            }
        } else {
            // Multiple generators would need nested iteration - this is complex
            Ok(quote! {
                std::collections::HashMap::new() // Complex dict comprehension with multiple generators not fully supported
            })
        }
    }
}

#[cfg(test)]
mod tests {
    // Note: These tests might need additional AST node implementations
    // create_parse_test!(test_simple_listcomp, "[x for x in range(5)]", "test.py");
    // create_parse_test!(test_listcomp_with_condition, "[x for x in range(10) if x % 2 == 0]", "test.py");
}