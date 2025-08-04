use python_ast::{parse, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    let code = std::fs::read_to_string("test_docstring.py")
        .expect("Failed to read test file");
    
    match parse(&code, "test_docstring.py") {
        Ok(ast) => {
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            let ctx = CodeGenContext::Module("test_docstring".to_string());
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    let generated_code = tokens.to_string();
                    
                    // Pretty format the code for better readability
                    let formatted = generated_code
                        .replace("# ! [doc = ", "//! ")
                        .replace("# [doc = ", "/// ")
                        .replace("] ", " ")
                        .replace("\" ]", "\"")
                        .replace("\" )", "\"")
                        .replace("{ ", "{\n")
                        .replace("} ", "}\n")
                        .replace("; ", ";\n");
                    
                    println!("Generated Rust code:");
                    println!("===================");
                    println!("{}", formatted);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}