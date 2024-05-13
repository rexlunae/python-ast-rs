//! Implements a Python-compatilble symbol table for Rust.

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;

use crate::tree::{ClassDef, FunctionDef, Import, ImportFrom};

//use log::{debug, info};

//use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};
use crate::tree::ExprType;

/// A stack of symbol tables of different scopes. Topmost is the current scope.
#[derive(Clone, Debug)]
pub struct SymbolTableScopes(VecDeque<SymbolTable>);

impl SymbolTableScopes {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, table: SymbolTable) {
        self.0.push_front(table);
    }

    pub fn pop(&mut self) -> Option<SymbolTable> {
        self.0.pop_front()
    }

    pub fn new_scope(&mut self) {
        self.0.push_front(SymbolTable::new());
    }

    pub fn insert(&mut self, key: String, value: SymbolTableNode) {
        if let Some(table) = self.0.front_mut() {
            table.insert(key, value);
        }
    }

    pub fn get(&self, key: &str) -> Option<&SymbolTableNode> {
        for table in self.0.iter() {
            if let Some(value) = table.get(key) {
                return Some(value);
            }
        }
        None
    }
}

impl Default for SymbolTableScopes {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub enum SymbolTableNode {
    Assign { position: usize, value: ExprType },
    ClassDef(ClassDef),
    FunctionDef(FunctionDef),
    Import(Import),
    ImportFrom(ImportFrom),
    Alias(String),
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
    pub symbols: HashMap<String, SymbolTableNode>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: SymbolTableNode) {
        self.symbols.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&SymbolTableNode> {
        self.symbols.get(key)
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (key, value) in self.symbols.iter() {
            s.push_str(&format!("{}: {:#?}\n", key, value));
        }
        write!(f, "{}", s)
    }
}
