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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::*;

    #[test]
    fn test_symbol_table_creation() {
        let table = SymbolTable::new();
        assert!(table.symbols.is_empty());
    }

    #[test]
    fn test_symbol_table_insert_and_get() {
        let mut table = SymbolTable::new();
        let node = SymbolTableNode::Alias("test_alias".to_string());
        
        table.insert("test_key".to_string(), node);
        
        let retrieved = table.get("test_key");
        assert!(retrieved.is_some());
        
        match retrieved.unwrap() {
            SymbolTableNode::Alias(alias) => assert_eq!(alias, "test_alias"),
            _ => panic!("Expected Alias node"),
        }
    }

    #[test]
    fn test_symbol_table_get_nonexistent() {
        let table = SymbolTable::new();
        assert!(table.get("nonexistent").is_none());
    }

    #[test]
    fn test_symbol_table_scopes_creation() {
        let scopes = SymbolTableScopes::new();
        assert_eq!(scopes.0.len(), 0);
    }

    #[test]
    fn test_symbol_table_scopes_push_pop() {
        let mut scopes = SymbolTableScopes::new();
        let table = SymbolTable::new();
        
        scopes.push(table);
        assert_eq!(scopes.0.len(), 1);
        
        let popped = scopes.pop();
        assert!(popped.is_some());
        assert_eq!(scopes.0.len(), 0);
    }

    #[test]
    fn test_symbol_table_scopes_new_scope() {
        let mut scopes = SymbolTableScopes::new();
        
        scopes.new_scope();
        assert_eq!(scopes.0.len(), 1);
        
        scopes.new_scope();
        assert_eq!(scopes.0.len(), 2);
    }

    #[test]
    fn test_symbol_table_scopes_insert_and_get() {
        let mut scopes = SymbolTableScopes::new();
        scopes.new_scope();
        
        let node = SymbolTableNode::Alias("scoped_alias".to_string());
        scopes.insert("test_key".to_string(), node);
        
        let retrieved = scopes.get("test_key");
        assert!(retrieved.is_some());
        
        match retrieved.unwrap() {
            SymbolTableNode::Alias(alias) => assert_eq!(alias, "scoped_alias"),
            _ => panic!("Expected Alias node"),
        }
    }

    #[test]
    fn test_symbol_table_scopes_nested_lookup() {
        let mut scopes = SymbolTableScopes::new();
        
        // Outer scope
        scopes.new_scope();
        let outer_node = SymbolTableNode::Alias("outer_alias".to_string());
        scopes.insert("outer_key".to_string(), outer_node);
        
        // Inner scope
        scopes.new_scope();
        let inner_node = SymbolTableNode::Alias("inner_alias".to_string());
        scopes.insert("inner_key".to_string(), inner_node);
        
        // Should find both keys
        assert!(scopes.get("inner_key").is_some());
        assert!(scopes.get("outer_key").is_some());
        
        // Inner scope should shadow outer scope for same key
        let shadow_node = SymbolTableNode::Alias("shadow_alias".to_string());
        scopes.insert("outer_key".to_string(), shadow_node);
        
        match scopes.get("outer_key").unwrap() {
            SymbolTableNode::Alias(alias) => assert_eq!(alias, "shadow_alias"),
            _ => panic!("Expected shadowed alias"),
        }
    }

    #[test]
    fn test_symbol_table_scopes_empty_get() {
        let scopes = SymbolTableScopes::new();
        assert!(scopes.get("any_key").is_none());
    }

    #[test]
    fn test_symbol_table_scopes_insert_no_scope() {
        let mut scopes = SymbolTableScopes::new();
        let node = SymbolTableNode::Alias("test".to_string());
        
        // Should not panic when inserting with no scopes
        scopes.insert("key".to_string(), node);
        
        // Should return None since no scopes exist
        assert!(scopes.get("key").is_none());
    }

    #[test]
    fn test_symbol_table_node_variants() {
        use crate::Name;
        
        // Test different node types
        let assign_node = SymbolTableNode::Assign {
            position: 42,
            value: ExprType::Name(Name { id: "test".to_string() }),
        };
        
        match assign_node {
            SymbolTableNode::Assign { position, .. } => assert_eq!(position, 42),
            _ => panic!("Expected Assign node"),
        }
        
        let alias_node = SymbolTableNode::Alias("alias_name".to_string());
        
        match alias_node {
            SymbolTableNode::Alias(name) => assert_eq!(name, "alias_name"),
            _ => panic!("Expected Alias node"),
        }
    }

    #[test]
    fn test_symbol_table_display() {
        let mut table = SymbolTable::new();
        let node = SymbolTableNode::Alias("test_display".to_string());
        table.insert("display_key".to_string(), node);
        
        let display_string = format!("{}", table);
        assert!(display_string.contains("display_key"));
        assert!(display_string.contains("test_display"));
    }

    #[test]
    fn test_symbol_table_scopes_default() {
        let scopes = SymbolTableScopes::default();
        assert_eq!(scopes.0.len(), 0);
    }

    #[test]
    fn test_symbol_table_default() {
        let table = SymbolTable::default();
        assert!(table.symbols.is_empty());
    }
}
