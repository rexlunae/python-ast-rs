use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    println!("Debugging with exact runtests.py content...\n");

    let code = std::fs::read_to_string("test_runtests_exact.py")
        .expect("Failed to read test file");
    
    match parse_enhanced(&code, "test_runtests_exact.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            // Let's examine the raw AST to see what's being parsed
            println!("Raw AST body count: {}", ast.raw.body.len());
            for (i, stmt) in ast.raw.body.iter().enumerate() {
                println!("Statement {}: {:?}", i, stmt.statement);
            }
            
            let ctx = CodeGenContext::Module("test_runtests_exact".to_string());
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("\n✅ Rust code generation successful!");
                    
                    let code_str = tokens.to_string();
                    println!("\n=== GENERATED RUST CODE ===");
                    println!("{}", code_str);
                    println!("=== END GENERATED CODE ===\n");
                    
                    // Check for multiple main functions
                    let main_count = code_str.matches("fn main").count();
                    println!("Number of 'fn main' occurrences: {}", main_count);
                    
                    // Check for our marker
                    if code_str.contains("__rython_main_block__") {
                        println!("✅ Found __rython_main_block__ marker");
                    } else {
                        println!("❌ Missing __rython_main_block__ marker - if __name__ detection failed");
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