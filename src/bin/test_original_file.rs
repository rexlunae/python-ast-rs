use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    println!("Testing with original problematic Python file...\n");

    let code = std::fs::read_to_string("/Users/tserica/pyperformance/runtests.py")
        .expect("Failed to read original test file");
    
    match parse_enhanced(&code, "runtests.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            let ctx = CodeGenContext::Module("runtests".to_string());
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    println!("\n=== GENERATED RUST CODE ===");
                    println!("{}", tokens);
                    println!("=== END GENERATED CODE ===\n");
                    
                    // Test if the generated code compiles
                    let temp_file = "/tmp/test_runtests.rs";
                    std::fs::write(temp_file, format!("{}", tokens))
                        .expect("Failed to write temp file");
                    
                    println!("Testing Rust compilation...");
                    match std::process::Command::new("rustc")
                        .args(&["--crate-type", "lib", temp_file])
                        .output() {
                        Ok(output) => {
                            if output.status.success() {
                                println!("✅ Generated Rust code compiles successfully!");
                            } else {
                                println!("❌ Rust compilation failed:");
                                println!("{}", String::from_utf8_lossy(&output.stderr));
                            }
                        }
                        Err(e) => {
                            println!("❌ Could not run rustc: {}", e);
                        }
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