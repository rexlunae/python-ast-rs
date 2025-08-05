use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen, AsyncRuntime};

fn main() {
    println!("Testing convenience methods for async runtime configuration...\n");

    let code = std::fs::read_to_string("test_async.py")
        .expect("Failed to read test file");
    
    // Test convenience methods
    let test_cases = vec![
        ("PythonOptions::default()", PythonOptions::default()),
        ("PythonOptions::with_tokio()", PythonOptions::with_tokio()),
        ("PythonOptions::with_async_std()", PythonOptions::with_async_std()),
        ("PythonOptions::with_smol()", PythonOptions::with_smol()),
        ("PythonOptions::with_custom_runtime()", PythonOptions::with_custom_runtime("my_async::main", "my_async")),
    ];

    for (name, options) in test_cases {
        println!("=== Testing {} ===", name);
        
        match parse_enhanced(&code, "test_async.py") {
            Ok(ast) => {
                let ctx = CodeGenContext::Module("test_async".to_string());
                let symbols = SymbolTableScopes::new();
                
                match ast.to_rust(ctx, options, symbols) {
                    Ok(tokens) => {
                        let code_str = tokens.to_string();
                        
                        // Extract and display the runtime information
                        if let Some(attr_start) = code_str.find("# [") {
                            if let Some(attr_end) = code_str[attr_start..].find("]") {
                                let attr_section = &code_str[attr_start..attr_start + attr_end + 1];
                                println!("   ✅ Attribute: {}", attr_section);
                            }
                        }
                        
                        // Find runtime import
                        let imports: Vec<&str> = code_str.split(" ; ")
                            .filter(|s| s.starts_with("use ") && !s.contains("stdpython"))
                            .collect();
                        
                        if !imports.is_empty() {
                            for import in imports {
                                println!("   ✅ Runtime import: {}", import.trim());
                            }
                        }
                    }
                    Err(e) => {
                        println!("   ❌ Code generation failed: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("   ❌ AST parsing failed: {}", e);
            }
        }
        println!();
    }
    
    // Test chaining methods
    println!("=== Testing method chaining ===");
    let mut options = PythonOptions::default();
    options.set_async_runtime(AsyncRuntime::AsyncStd);
    
    match parse_enhanced(&code, "test_async.py") {
        Ok(ast) => {
            let ctx = CodeGenContext::Module("test_async".to_string());
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    let code_str = tokens.to_string();
                    if code_str.contains("async_std :: main") {
                        println!("✅ Method chaining works - async-std runtime configured");
                    } else {
                        println!("❌ Method chaining failed");
                    }
                }
                Err(e) => {
                    println!("❌ Code generation failed: {}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ AST parsing failed: {}", e);
        }
    }
}