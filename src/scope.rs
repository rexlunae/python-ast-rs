use std::collections::HashMap;
use std::default::Default;

/// Represents a single symbol within a scope.
#[derive(Clone, Debug, Default)]
pub enum Symbol {
    SubScope(Box<Scope>),
    Class(),
    Variable(),
    Const(String),
    #[default]
    Unknown,
}

/// Python uses LEGB scope: Local, Enclosing, Global, and Built-in.
/// Local scope consists of local variables inside a function. Names in the local scope may change new declarations overwrite older ones.
/// Enclosing scope is the scope of a containing function with inner/nested functions.
/// Global is the global scope. Names within the global namespace must be unique.
/// Built-in is basically a special scope for elements that are built into Python.
#[derive(Clone, Debug, Default)]
pub enum Scope {
    #[default]
    None,
    Local(HashMap<String, Symbol>),
    Global(HashMap<String, Symbol>),
}
