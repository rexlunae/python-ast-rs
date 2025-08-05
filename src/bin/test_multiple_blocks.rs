use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    println!("Testing multiple if __name__ == '__main__' blocks...\n");

    let code = std::fs::read_to_string("test_multiple_main_blocks.py")
        .expect("Failed to read test file");
    
    match parse_enhanced(&code, "test_multiple_main_blocks.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            let ctx = CodeGenContext::Module("test_multiple_main_blocks".to_string());
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    
                    let code_str = tokens.to_string();
                    println!("\n=== GENERATED RUST CODE ===");
                    println!("{}", code_str);
                    println!("=== END GENERATED CODE ===\n");
                    
                    // Check for consolidation
                    let main_count = code_str.matches("fn main").count();
                    println!("Number of 'fn main' occurrences: {}", main_count);
                    
                    if main_count == 1 {
                        println!("✅ Multiple main blocks consolidated successfully!");
                    } else {
                        println!("❌ Multiple main functions still exist");
                    }
                    
                    // Check that both blocks are included
                    if code_str.contains("python_main ()") && 
                       code_str.contains("another_function ()") &&
                       code_str.contains("First main block") &&
                       code_str.contains("Second main block") {
                        println!("✅ All main block content consolidated");
                    } else {
                        println!("❌ Some main block content missing");
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