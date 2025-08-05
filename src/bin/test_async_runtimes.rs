use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen, AsyncRuntime};

fn main() {
    println!("Testing different async runtimes...\n");

    let code = std::fs::read_to_string("test_async.py")
        .expect("Failed to read test file");
    
    // Test different async runtimes
    let runtimes = vec![
        ("Tokio (default)", AsyncRuntime::Tokio),
        ("async-std", AsyncRuntime::AsyncStd),
        ("smol", AsyncRuntime::Smol),
        ("Custom (async-std-like)", AsyncRuntime::Custom {
            attribute: "async_std::main".to_string(),
            import: "async_std".to_string(),
        }),
        ("Custom (custom runtime)", AsyncRuntime::Custom {
            attribute: "my_runtime::main".to_string(),
            import: "my_runtime".to_string(),
        }),
    ];

    for (name, runtime) in runtimes {
        println!("=== Testing {} ===", name);
        
        match parse_enhanced(&code, "test_async.py") {
            Ok(ast) => {
                println!("✅ Python AST parsed successfully!");
                
                let ctx = CodeGenContext::Module("test_async".to_string());
                let mut options = PythonOptions::default();
                options.async_runtime = runtime;
                let symbols = SymbolTableScopes::new();
                
                match ast.to_rust(ctx, options, symbols) {
                    Ok(tokens) => {
                        println!("✅ Rust code generation successful!");
                        
                        let code_str = tokens.to_string();
                        
                        // Extract key parts for verification
                        if let Some(import_start) = code_str.find("use ") {
                            if let Some(import_end) = code_str[import_start..].find(" ;") {
                                let import_section = &code_str[import_start..import_start + import_end + 2];
                                println!("   Import: {}", import_section);
                            }
                        }
                        
                        if let Some(attr_start) = code_str.find("# [") {
                            if let Some(attr_end) = code_str[attr_start..].find("]") {
                                let attr_section = &code_str[attr_start..attr_start + attr_end + 1];
                                println!("   Attribute: {}", attr_section);
                            }
                        }
                        
                        // Check for async main
                        if code_str.contains("async fn main") {
                            println!("   ✅ Async main function detected");
                        } else {
                            println!("   ❌ Async main function missing");
                        }
                        
                        println!("   Generated: {} characters", code_str.len());
                    }
                    Err(e) => {
                        println!("❌ Rust code generation failed: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("❌ Python AST parsing failed: {}", e);
            }
        }
        
        println!();
    }
    
    // Test with non-async code to ensure regular main is generated
    println!("=== Testing with non-async code ===");
    let sync_code = r#"
def hello():
    print("Hello, World!")

if __name__ == "__main__":
    hello()
"#;
    
    match parse_enhanced(sync_code, "test_sync.py") {
        Ok(ast) => {
            let ctx = CodeGenContext::Module("test_sync".to_string());
            let mut options = PythonOptions::default();
            options.async_runtime = AsyncRuntime::AsyncStd; // Use non-default to verify it doesn't affect sync code
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    let code_str = tokens.to_string();
                    
                    if !code_str.contains("async fn main") && code_str.contains("fn main") {
                        println!("✅ Regular (non-async) main function generated for sync code");
                    } else {
                        println!("❌ Expected regular main function, got async");
                    }
                    
                    if !code_str.contains("use async_std") {
                        println!("✅ No async runtime import for sync code");  
                    } else {
                        println!("❌ Unexpected async runtime import for sync code");
                    }
                }
                Err(e) => {
                    println!("❌ Rust code generation failed: {}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ Python AST parsing failed: {}", e);
        }
    }
}