use python_ast::{parse, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    let code = r#"#!/usr/bin/env python3

import sys
import os
import subprocess

# Test the exact same code pattern that was failing  
python = sys.executable
if python != sys.executable:
    os.execv(python, [python] + sys.argv)

proc = subprocess.run([sys.executable, "-u", "-m", "pyperformance.tests"], 
                      cwd=os.path.dirname(__file__))
sys.exit(proc.returncode)
"#;
    
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