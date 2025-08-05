use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    println!("Debugging the actual runtests.py multiple main issue...\n");

    let code = std::fs::read_to_string("/Users/tserica/pyperformance/runtests.py")
        .expect("Failed to read runtests.py");
    
    match parse_enhanced(&code, "runtests.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            let ctx = CodeGenContext::Module("runtests".to_string());
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    
                    let code_str = tokens.to_string();
                    println!("\n=== GENERATED RUST CODE ===");
                    
                    // Split into lines and number them to match the error
                    let lines: Vec<&str> = code_str.split(';').collect();
                    for (i, line) in lines.iter().enumerate() {
                        let line_trimmed = line.trim();
                        if !line_trimmed.is_empty() {
                            println!("{:2}: {}", i + 1, line_trimmed);
                        }
                    }
                    
                    println!("=== END GENERATED CODE ===\n");
                    
                    // Analyze the issue
                    let main_positions: Vec<_> = code_str.match_indices("fn main").collect();
                    println!("Found {} 'fn main' occurrences:", main_positions.len());
                    for (pos, _) in main_positions {
                        let before = &code_str[..pos];
                        let line_num = before.matches(';').count() + 1;
                        println!("  - At position {} (around line {})", pos, line_num);
                    }
                    
                    // Check for our special marker
                    if code_str.contains("__rython_main_block__") {
                        println!("\n✅ Found __rython_main_block__ marker");
                    } else {
                        println!("\n❌ Missing __rython_main_block__ marker");
                    }
                    
                    // Check for python_main
                    if code_str.contains("python_main") {
                        println!("✅ Found python_main function");
                    } else {
                        println!("❌ Missing python_main function");
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