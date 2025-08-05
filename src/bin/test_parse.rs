use python_ast::{parse, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    let code = std::fs::read_to_string("test_syntax_errors.py")
        .expect("Failed to read test file");
    
    println!("Testing comprehensive Python code parsing...");
    
    match parse(&code, "test_comprehensive.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            println!("📊 Module has {} statements", ast.raw.body.len());
            
            for (i, stmt) in ast.raw.body.iter().enumerate() {
                println!("Statement {}: {:?}", i, std::mem::discriminant(&stmt.statement));
            }
            
            // Try to generate Rust code
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            let ctx = CodeGenContext::Module("test_comprehensive".to_string());
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    println!("Generated {} characters", tokens.to_string().len());
                    println!("\n=== GENERATED RUST CODE ===");
                    
                    // Print raw tokens for debugging
                    let code = tokens.to_string();
                    println!("{}", code);
                    println!("\n=== END GENERATED CODE ===\n");
                }
                Err(e) => {
                    println!("❌ Rust code generation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Python AST parsing failed: {}", e);
        }
    }
}