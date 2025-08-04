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

impl Scope {
    pub fn new_local() -> Self {
        Scope::Local(HashMap::new())
    }

    pub fn new_global() -> Self {
        Scope::Global(HashMap::new())
    }

    pub fn insert(&mut self, key: String, symbol: Symbol) -> Option<Symbol> {
        match self {
            Scope::Local(map) | Scope::Global(map) => map.insert(key, symbol),
            Scope::None => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&Symbol> {
        match self {
            Scope::Local(map) | Scope::Global(map) => map.get(key),
            Scope::None => None,
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            Scope::Local(map) | Scope::Global(map) => map.contains_key(key),
            Scope::None => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Scope::Local(map) | Scope::Global(map) => map.len(),
            Scope::None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_default() {
        let symbol = Symbol::default();
        matches!(symbol, Symbol::Unknown);
    }

    #[test]
    fn test_symbol_variants() {
        let class_symbol = Symbol::Class();
        let variable_symbol = Symbol::Variable();
        let const_symbol = Symbol::Const("test_value".to_string());
        let unknown_symbol = Symbol::Unknown;

        matches!(class_symbol, Symbol::Class());
        matches!(variable_symbol, Symbol::Variable());
        matches!(const_symbol, Symbol::Const(_));
        matches!(unknown_symbol, Symbol::Unknown);
    }

    #[test]
    fn test_symbol_subscope() {
        let inner_scope = Box::new(Scope::new_local());
        let subscope_symbol = Symbol::SubScope(inner_scope);
        
        match subscope_symbol {
            Symbol::SubScope(scope) => {
                assert!(scope.is_empty());
            }
            _ => panic!("Expected SubScope symbol"),
        }
    }

    #[test]
    fn test_scope_default() {
        let scope = Scope::default();
        matches!(scope, Scope::None);
        assert!(scope.is_empty());
    }

    #[test]
    fn test_scope_new_local() {
        let scope = Scope::new_local();
        matches!(scope, Scope::Local(_));
        assert!(scope.is_empty());
    }

    #[test]
    fn test_scope_new_global() {
        let scope = Scope::new_global();
        matches!(scope, Scope::Global(_));
        assert!(scope.is_empty());
    }

    #[test]
    fn test_scope_insert_and_get_local() {
        let mut scope = Scope::new_local();
        let symbol = Symbol::Variable();
        
        let previous = scope.insert("test_var".to_string(), symbol);
        assert!(previous.is_none());
        assert_eq!(scope.len(), 1);
        
        let retrieved = scope.get("test_var");
        assert!(retrieved.is_some());
        matches!(retrieved.unwrap(), Symbol::Variable());
    }

    #[test]
    fn test_scope_insert_and_get_global() {
        let mut scope = Scope::new_global();
        let symbol = Symbol::Class();
        
        let previous = scope.insert("TestClass".to_string(), symbol);
        assert!(previous.is_none());
        assert_eq!(scope.len(), 1);
        
        let retrieved = scope.get("TestClass");
        assert!(retrieved.is_some());
        matches!(retrieved.unwrap(), Symbol::Class());
    }

    #[test]
    fn test_scope_insert_overwrite() {
        let mut scope = Scope::new_local();
        let symbol1 = Symbol::Variable();
        let symbol2 = Symbol::Const("new_value".to_string());
        
        scope.insert("test_key".to_string(), symbol1);
        let previous = scope.insert("test_key".to_string(), symbol2);
        
        assert!(previous.is_some());
        matches!(previous.unwrap(), Symbol::Variable());
        assert_eq!(scope.len(), 1);
        
        let current = scope.get("test_key");
        match current.unwrap() {
            Symbol::Const(value) => assert_eq!(value, "new_value"),
            _ => panic!("Expected Const symbol"),
        }
    }

    #[test]
    fn test_scope_none_operations() {
        let mut scope = Scope::None;
        let symbol = Symbol::Variable();
        
        let result = scope.insert("test".to_string(), symbol);
        assert!(result.is_none());
        
        let retrieved = scope.get("test");
        assert!(retrieved.is_none());
        
        assert!(!scope.contains_key("test"));
        assert_eq!(scope.len(), 0);
        assert!(scope.is_empty());
    }

    #[test]
    fn test_scope_contains_key() {
        let mut scope = Scope::new_local();
        let symbol = Symbol::Variable();
        
        assert!(!scope.contains_key("test_var"));
        
        scope.insert("test_var".to_string(), symbol);
        assert!(scope.contains_key("test_var"));
        assert!(!scope.contains_key("other_var"));
    }

    #[test]
    fn test_scope_len_and_empty() {
        let mut scope = Scope::new_local();
        
        assert_eq!(scope.len(), 0);
        assert!(scope.is_empty());
        
        scope.insert("var1".to_string(), Symbol::Variable());
        assert_eq!(scope.len(), 1);
        assert!(!scope.is_empty());
        
        scope.insert("var2".to_string(), Symbol::Class());
        assert_eq!(scope.len(), 2);
        assert!(!scope.is_empty());
    }

    #[test]
    fn test_scope_get_nonexistent() {
        let scope = Scope::new_local();
        assert!(scope.get("nonexistent").is_none());
        
        let global_scope = Scope::new_global();
        assert!(global_scope.get("nonexistent").is_none());
        
        let none_scope = Scope::None;
        assert!(none_scope.get("nonexistent").is_none());
    }

    #[test]
    fn test_nested_scopes() {
        let mut inner_scope = Scope::new_local();
        inner_scope.insert("inner_var".to_string(), Symbol::Variable());
        
        let mut outer_scope = Scope::new_global();
        let subscope_symbol = Symbol::SubScope(Box::new(inner_scope));
        outer_scope.insert("inner_function".to_string(), subscope_symbol);
        
        assert_eq!(outer_scope.len(), 1);
        
        match outer_scope.get("inner_function").unwrap() {
            Symbol::SubScope(scope) => {
                assert_eq!(scope.len(), 1);
                assert!(scope.contains_key("inner_var"));
            }
            _ => panic!("Expected SubScope symbol"),
        }
    }
}
