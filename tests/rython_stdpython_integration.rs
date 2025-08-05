use python_ast::{parse, CodeGen, CodeGenContext, PythonOptions, SymbolTableScopes};
use test_log::test;

#[test]
fn test_print_function_with_stdpython() {
    let code = r#"print("Hello, world!")"#;
    let module = parse(code, "test.py").unwrap();
    
    let options = PythonOptions::default();
    let rust_code = module.to_rust(
        CodeGenContext::Module("test".to_string()),
        options,
        SymbolTableScopes::new(),
    ).unwrap();
    
    let rust_str = rust_code.to_string();
    assert!(rust_str.contains("use stdpython :: *"));
    assert!(rust_str.contains("print"));
}

#[test]
fn test_basic_arithmetic_with_stdpython() {
    let code = r#"
result = 5 + 3
print(result)
"#;
    let module = parse(code, "test.py").unwrap();
    
    let options = PythonOptions::default();
    let rust_code = module.to_rust(
        CodeGenContext::Module("test".to_string()),
        options,
        SymbolTableScopes::new(),
    ).unwrap();
    
    let rust_str = rust_code.to_string();
    assert!(rust_str.contains("use stdpython :: *"));
}

#[test]
fn test_list_operations_with_stdpython() {
    let code = r#"
my_list = [1, 2, 3]
print(len(my_list))
"#;
    let module = parse(code, "test.py").unwrap();
    
    let options = PythonOptions::default();
    let rust_code = module.to_rust(
        CodeGenContext::Module("test".to_string()),
        options,
        SymbolTableScopes::new(),
    ).unwrap();
    
    let rust_str = rust_code.to_string();
    assert!(rust_str.contains("use stdpython :: *"));
    assert!(rust_str.contains("len"));
}

#[test]
fn test_function_definition_with_stdpython() {
    let code = r#"
def greet(name):
    return "Hello, " + name

result = greet("Python")
print(result)
"#;
    let module = parse(code, "test.py").unwrap();
    
    let options = PythonOptions::default();
    let rust_code = module.to_rust(
        CodeGenContext::Module("test".to_string()),
        options,
        SymbolTableScopes::new(),
    ).unwrap();
    
    let rust_str = rust_code.to_string();
    assert!(rust_str.contains("use stdpython :: *"));
}

#[test]
fn test_nostd_mode() {
    let code = r#"print("Hello, world!")"#;
    let module = parse(code, "test.py").unwrap();
    
    let mut options = PythonOptions::default();
    options.with_std_python = false;
    
    let rust_code = module.to_rust(
        CodeGenContext::Module("test".to_string()),
        options,
        SymbolTableScopes::new(),
    ).unwrap();
    
    let rust_str = rust_code.to_string();
    assert!(!rust_str.contains("use stdpython :: *"));
}