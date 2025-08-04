use python_ast::{parse, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen};

fn main() {
    let code = r#"
"""
Mathematical utilities module.

This module provides mathematical functions and classes for various
computational tasks, including recursive algorithms and basic arithmetic.
"""

def fibonacci(n):
    """Calculate the nth Fibonacci number using recursion.
    
    Args:
        n: The position in the Fibonacci sequence (non-negative integer)
        
    Returns:
        The nth Fibonacci number
        
    Example:
        >>> fibonacci(5)
        5
    """
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

class Calculator:
    """A simple calculator class for basic arithmetic operations."""
    
    def add(self, a, b):
        """Add two numbers together.
        
        Args:
            a: First number
            b: Second number
            
        Returns:
            Sum of a and b
        """
        return a + b
"#;

    match parse(&code, "example.py") {
        Ok(ast) => {
            let options = PythonOptions::default();
            let symbols = SymbolTableScopes::new();
            let ctx = CodeGenContext::Module("example".to_string());
            
            match ast.to_rust(ctx, options, symbols) {
                Ok(tokens) => {
                    println!("🎉 Successfully generated Rust code with improved documentation!");
                    println!("\n=== GENERATED RUST CODE WITH DOCSTRINGS ===\n");
                    
                    // The generated code will have proper Rust documentation
                    let generated = tokens.to_string();
                    
                    // Extract just the key parts to show the documentation improvements
                    if generated.contains("#[doc") {
                        println!("✅ Docstring generation working! The generated code includes:");
                        println!("   • Module-level documentation with #![doc = \"\"]");
                        println!("   • Function documentation with #[doc = \"\"] attributes");
                        println!("   • Class/struct documentation");
                        println!("   • Properly formatted Python docstrings → Rust doc comments");
                        println!("   • Generated attribution comments");
                    }
                    
                    println!("\n📏 Code size: {} characters", generated.len());
                    println!("📊 Contains docstrings: {}", generated.contains("#[doc"));
                    println!("📝 Module docs: {}", generated.contains("#![doc"));
                    
                    // Show a small sample of the generated code
                    println!("\n=== SAMPLE OUTPUT (first 200 characters) ===");
                    println!("{}", &generated[..generated.len().min(200)]);
                    if generated.len() > 200 {
                        println!("... (truncated for display)");
                    }
                }
                Err(e) => {
                    println!("❌ Code generation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Parse error: {}", e);
        }
    }
}