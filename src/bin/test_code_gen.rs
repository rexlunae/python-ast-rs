use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    println!("Testing code generation fixes...\n");

    let code = std::fs::read_to_string("test_code_generation.py")
        .expect("Failed to read test file");
    
    match parse_enhanced(&code, "test_code_generation.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            let ctx = CodeGenContext::Module("test_code_generation".to_string());
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    println!("\n=== GENERATED RUST CODE ===");
                    println!("{}", tokens);
                    println!("=== END GENERATED CODE ===\n");
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