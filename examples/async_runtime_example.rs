use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen, AsyncRuntime};

fn main() {
    let python_code = r#"
import asyncio

async def fetch_data():
    await asyncio.sleep(1)
    return "data"

async def main():
    result = await fetch_data()
    print(result)

if __name__ == "__main__":
    asyncio.run(main())
"#;

    // Example 1: Using tokio (default)
    println!("=== Example 1: Tokio (default) ===");
    let options = PythonOptions::default(); // tokio is the default
    generate_and_display(python_code, options);

    // Example 2: Using async-std
    println!("\n=== Example 2: async-std ===");
    let options = PythonOptions::with_async_std();
    generate_and_display(python_code, options);

    // Example 3: Using smol
    println!("\n=== Example 3: smol ===");
    let options = PythonOptions::with_smol();
    generate_and_display(python_code, options);

    // Example 4: Using custom runtime
    println!("\n=== Example 4: Custom runtime ===");
    let options = PythonOptions::with_custom_runtime("my_runtime::main", "my_runtime");
    generate_and_display(python_code, options);

    // Example 5: Programmatically setting runtime
    println!("\n=== Example 5: Programmatic configuration ===");
    let mut options = PythonOptions::default();
    options.set_async_runtime(AsyncRuntime::AsyncStd);
    generate_and_display(python_code, options);
}

fn generate_and_display(python_code: &str, options: PythonOptions) {
    match parse_enhanced(python_code, "example.py") {
        Ok(ast) => {
            let ctx = CodeGenContext::Module("example".to_string());
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    let code_str = tokens.to_string();
                    
                    // Pretty print key parts
                    println!("Generated Rust code:");
                    
                    // Extract imports
                    let parts: Vec<&str> = code_str.split(" ; ").collect();
                    for part in &parts {
                        if part.starts_with("use ") {
                            println!("  {}", part.trim());
                        }
                    }
                    
                    // Find main function
                    if let Some(main_start) = code_str.find("# [") {
                        if let Some(main_end) = code_str[main_start..].find("async fn main") {
                            if let Some(brace) = code_str[main_start + main_end..].find(" {") {
                                let main_signature = &code_str[main_start..main_start + main_end + brace + 2];
                                println!("  {}", main_signature.trim());
                                println!("    // ... main function body ...");
                                println!("  }}");
                            }
                        }
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