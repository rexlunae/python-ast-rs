use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen, AsyncRuntime};

fn main() {
    println!("Detailed output test for async-std runtime...\n");

    let code = std::fs::read_to_string("test_async.py")
        .expect("Failed to read test file");
    
    match parse_enhanced(&code, "test_async.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            let ctx = CodeGenContext::Module("test_async".to_string());
            let mut options = PythonOptions::default();
            options.async_runtime = AsyncRuntime::AsyncStd;
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    println!("\n=== FULL GENERATED RUST CODE ===");
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