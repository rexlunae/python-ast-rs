use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    println!("Testing async improvements and main function consolidation...\n");

    let code = std::fs::read_to_string("test_async.py")
        .expect("Failed to read test file");
    
    match parse_enhanced(&code, "test_async.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            let ctx = CodeGenContext::Module("test_async".to_string());
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    println!("\n=== GENERATED RUST CODE ===");
                    println!("{}", tokens);
                    println!("=== END GENERATED CODE ===\n");
                    
                    // Check for key improvements
                    let code_str = tokens.to_string();
                    
                    // Check for tokio usage
                    if code_str.contains("use tokio") {
                        println!("✅ Tokio import detected");
                    } else {
                        println!("❌ Tokio import missing");
                    }
                    
                    // Check for async main
                    if code_str.contains("# [tokio :: main]") && code_str.contains("async fn main") {
                        println!("✅ Async main function with tokio::main detected");
                    } else if code_str.contains("#[tokio::main]") && code_str.contains("async fn main") {
                        println!("✅ Async main function with tokio::main detected");
                    } else {
                        println!("❌ Async main function missing");
                        println!("   Looking for: '#[tokio::main]' and 'async fn main'");
                        println!("   tokio::main found: {}", code_str.contains("tokio :: main"));
                        println!("   async fn main found: {}", code_str.contains("async fn main"));
                    }
                    
                    // Check for single main function (not multiple)
                    let main_count = code_str.matches("fn main").count();
                    if main_count == 1 {
                        println!("✅ Single main function generated");
                    } else {
                        println!("❌ Multiple main functions found: {}", main_count);
                    }
                    
                    // Check for async function generation
                    if code_str.contains("async fn") {
                        println!("✅ Async functions detected in output");
                    } else {
                        println!("❌ No async functions in output");
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