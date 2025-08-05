use python_ast::{parse, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <python_file>", args[0]);
        return;
    }
    
    let filename = &args[1];
    let code = std::fs::read_to_string(filename)
        .expect("Failed to read test file");
    
    println!("Testing main function conflict handling for: {}", filename);
    
    match parse(&code, filename) {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            println!("📊 Module has {} statements", ast.raw.body.len());
            
            for (i, stmt) in ast.raw.body.iter().enumerate() {
                println!("Statement {}: {:?}", i, std::mem::discriminant(&stmt.statement));
            }
            
            // Try to generate Rust code
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            let ctx = CodeGenContext::Module("test_main_conflicts".to_string());
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("✅ Rust code generation successful!");
                    println!("Generated {} characters", tokens.to_string().len());
                    println!("\n=== GENERATED RUST CODE ===");
                    
                    // Print raw tokens for debugging
                    let code = tokens.to_string();
                    println!("{}", code);
                    println!("\n=== END GENERATED CODE ===\n");
                    
                    // Analyze the generated code for main function issues
                    analyze_main_function_handling(&code);
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

fn analyze_main_function_handling(generated_code: &str) {
    println!("=== MAIN FUNCTION ANALYSIS ===");
    
    // Check for main function definitions
    let main_function_count = generated_code.matches("fn main(").count() + 
                             generated_code.matches("async fn main(").count();
    let python_main_count = generated_code.matches("fn python_main(").count() + 
                           generated_code.matches("async fn python_main(").count();
    
    println!("🔍 Found {} main() functions", main_function_count);
    println!("🔍 Found {} python_main() functions", python_main_count);
    
    if main_function_count > 1 {
        println!("⚠️  WARNING: Multiple main() functions detected!");
    }
    
    if python_main_count > 0 {
        println!("✅ User's main function correctly renamed to python_main()");
    }
    
    // Check for call references
    let main_calls = generated_code.matches("main()").count() + 
                    generated_code.matches("main (").count();
    let python_main_calls = generated_code.matches("python_main()").count() + 
                           generated_code.matches("python_main (").count();
    
    println!("🔍 Found {} calls to main()", main_calls);
    println!("🔍 Found {} calls to python_main()", python_main_calls);
    
    if main_calls > 0 && python_main_count > 0 {
        println!("⚠️  WARNING: Found calls to main() but user function was renamed to python_main()");
        println!("This may indicate incorrect reference updating!");
    }
    
    println!("=== END ANALYSIS ===\n");
}