use python_ast::{parse_enhanced, StatementType};

fn main() {
    println!("Debugging if __name__ == '__main__' detection...\n");

    let code = r#"
if __name__ == "__main__":
    print("hello")
"#;
    
    match parse_enhanced(code, "test.py") {
        Ok(ast) => {
            println!("✅ Python AST parsed successfully!");
            
            // Find the if statement
            for (i, stmt) in ast.raw.body.iter().enumerate() {
                if let StatementType::If(if_stmt) = &stmt.statement {
                    println!("If statement {}: {:?}", i, if_stmt.test);
                    
                    let test_str = format!("{:?}", if_stmt.test);
                    println!("Debug string: {}", test_str);
                    println!("Contains __name__: {}", test_str.contains("__name__"));
                    println!("Contains __main__: {}", test_str.contains("__main__"));
                }
            }
        },
        Err(e) => {
            println!("❌ Python AST parsing failed: {}", e);
        }
    }
}