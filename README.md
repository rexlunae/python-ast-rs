# python-ast-rs

[![Crates.io](https://img.shields.io/crates/v/python-ast.svg)](https://crates.io/crates/python-ast)
[![Documentation](https://docs.rs/python-ast/badge.svg)](https://docs.rs/python-ast)
[![License](https://img.shields.io/crates/l/python-ast.svg)](LICENSE)

A Rust library for parsing Python code into Abstract Syntax Trees (AST) and experimentally transpiling Python to Rust. This library leverages Python's own `ast` module via PyO3 to ensure compatibility with the reference Python implementation.

## ✨ Features

- **🐍 Python AST Parsing**: Parse any valid Python code into Rust data structures
- **🔄 Comprehensive Node Support**: Supports expressions, statements, functions, classes, and more
- **📚 Rich Documentation**: Automatically extracts and converts Python docstrings to Rust documentation
- **🦀 Generic Rust Code Generation**: Transpile Python code to highly generic Rust using trait bounds
- **🔧 Extensible**: Built with traits and macros for easy extension

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.82+ (2024 edition)
- **Python**: 3.8+ with development headers
- **PyO3 Dependencies**: Automatically handled via PyO3's auto-initialize feature

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
python-ast = "1.0.0"
```

### Basic Usage

#### Parsing Python Code

```rust
use python_ast::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse Python source code
    let python_code = "def hello():\n    return 'world'";
    
    // Parse into AST
    let ast = parse(python_code, "example.py")?;
    
    println!("Parsed {} statements", ast.raw.body.len());
    println!("AST: {:#?}", ast);
    
    Ok(())
}
```

#### Experimental Code Generation

```rust
use python_ast::{parse, CodeGen, CodeGenContext, PythonOptions, SymbolTableScopes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let python_code = r#"
def fibonacci(n):
    """Calculate the nth Fibonacci number."""
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)
"#;
    
    // Parse Python code
    let ast = parse(python_code, "fibonacci.py")?;
    
    // Generate Rust code (experimental)
    let options = PythonOptions::default();
    let symbols = SymbolTableScopes::new();
    let context = CodeGenContext::Module("fibonacci".to_string());
    
    match ast.to_rust(context, options, symbols) {
        Ok(rust_code) => {
            println!("Generated Rust code:");
            println!("{}", rust_code);
            /* Output would be:
            use stdpython::*;
            #[doc = "Calculate the nth Fibonacci number."]
            pub fn fibonacci(n: impl Into<PyObject>) -> impl Into<PyObject> {
                "Calculate the nth Fibonacci number.";
                if ((n) <= (1)) {
                    return n
                };
                return (fibonacci((n) - (1))) + (fibonacci((n) - (2)));
            }
            */
        }
        Err(e) => {
            println!("Code generation failed: {}", e);
        }
    }
    
    Ok(())
}
```

## 🏗️ Architecture

### Core Components

- **Parser** (`src/parser/`): Python AST extraction via PyO3
- **AST Nodes** (`src/ast/tree/`): Rust representations of Python AST nodes
- **Code Generation** (`src/codegen/`): Experimental Python-to-Rust transpiler
- **Utilities** (`src/traits.rs`, `src/macros.rs`): Helper traits and macros

### Supported Python Constructs

#### ✅ Fully Supported
- **Expressions**: Binary/unary operations, comparisons, function calls, literals
- **Statements**: Function definitions, class definitions, assignments, imports
- **Control Flow**: If statements, for/while loops (basic support)
- **Data Structures**: Lists, tuples, dictionaries, sets
- **Advanced**: Lambda expressions, conditional expressions, subscripting

#### ⚠️ Experimental/Limited Support
- **Async/Await**: Parsing supported, code generation experimental
- **Decorators**: Parsing supported, code generation limited
- **Exception Handling**: Parsing supported, code generation experimental
- **Comprehensions**: List comprehensions not yet fully supported

#### ❌ Not Yet Supported
- **f-strings**: JoinedStr/FormattedValue nodes
- **Walrus Operator**: Advanced assignment expressions
- **Match Statements**: Python 3.10+ pattern matching

## 📚 Documentation Features

The library automatically extracts and converts Python docstrings:

```python
def calculate_area(radius):
    """Calculate the area of a circle.
    
    Args:
        radius: The radius of the circle
        
    Returns:
        The area of the circle
    """
    return 3.14159 * radius * radius
```

Becomes:

```rust
use std::ops::Mul;

/// Calculate the area of a circle.
/// 
/// # Arguments
/// radius: The radius of the circle
/// 
/// # Returns
/// The area of the circle
pub fn calculate_area<T>(radius: T) -> T 
where 
    T: Copy + Mul<Output = T> + From<f64>,
{
    // Generated from Python AST: return 3.14159 * radius * radius
    T::from(3.14159) * radius * radius
}
```

## 🧪 Testing

Run the test suite:

```bash
# Run all integration tests
cargo test
# Run unit tests.
cargo test --lib

# Run specific test categories
cargo test ast::tree        # AST node tests
cargo test parser            # Parser tests
cargo test --lib --quiet    # Library tests only
```

**Current Test Status**: 122/129 tests passing
- Some advanced features (async, decorators, complex expressions) have known test failures
- Basic parsing and code generation work reliably

## 🔧 Development

### Building from Source

```bash
git clone https://github.com/rexlunae/python-ast-rs.git
cd python-ast-rs

# Build the library
cargo build

# Run examples
cargo run --bin readme_example
```

### Contributing

1. **Parse Tree Coverage**: Help implement missing Python AST node types
2. **Code Generation**: Improve the experimental Rust code generation
3. **Testing**: Add test cases for edge cases and new features
4. **Documentation**: Improve code documentation and examples

## 🆕 Recent Improvements (v1.0.0)

- **Generic Code Generation**: Functions now use trait bounds like `impl Into<PyObject>`, `impl AsRef<str>`
- **Enhanced Documentation**: Python docstrings now convert to proper Rust doc comments
- **Flexible Parameters**: Variadic args use `impl IntoIterator<Item = impl Into<PyObject>>`
- **Generic Keyword Args**: **kwargs use `impl IntoIterator<Item = (impl AsRef<str>, impl Into<PyObject>)>`
- **Expanded AST Coverage**: Added support for Lambda, IfExp, Dict, Set, Tuple, Subscript expressions
- **Control Flow**: Implemented If, For, While statement parsing and generation
- **Improved Parsing**: Fixed List expression parsing and enhanced error handling
- **Developer Experience**: Added comprehensive macros and traits for extensibility

## ⚠️ Current Limitations

- **Experimental Status**: This project is in active development and APIs may change
- **Python Dependency**: Requires Python runtime via PyO3 for AST parsing
- **Code Generation Quality**: Generated Rust code is not production-ready
- **Performance**: Not optimized for large codebases
- **Error Handling**: Some parsing failures result in panics rather than graceful errors

## 🎯 Goals & Vision

The long-term goal is to create a **fully-compiled Python-like language** that:
- Maintains Python's syntax and semantics as closely as possible
- Provides Rust's static typing and memory safety guarantees
- Enables fearless concurrency and high performance
- Serves as a bridge for migrating Python codebases to Rust

Currently, this should be viewed as a **proof of concept** and research tool rather than a production-ready solution.

## 📖 Examples

### Parsing Different Python Constructs

```rust
use python_ast::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a simple function
    let ast = parse("def hello(): return 'world'", "hello.py")?;
    
    // Parse a class with methods
    let ast = parse(r#"
class Calculator:
    def add(self, a, b):
        return a + b
"#, "calc.py")?;
    
    // Parse control flow
    let ast = parse(r#"
for i in range(10):
    if i % 2 == 0:
        print(i)
"#, "loop.py")?;
    
    Ok(())
}
```

### Advanced Code Generation with Generic Parameters

The library generates highly generic Rust code using trait bounds for maximum flexibility:

```rust
use python_ast::{parse, CodeGen, CodeGenContext, PythonOptions, SymbolTableScopes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let python_code = r#"
def process_data(name, values, *args, **kwargs):
    """Process data with flexible parameter types."""
    return f"Processing {name} with {len(values)} items"
"#;
    
    let ast = parse(python_code, "example.py")?;
    let options = PythonOptions::default();
    let symbols = SymbolTableScopes::new();
    let context = CodeGenContext::Module("example".to_string());
    
    match ast.to_rust(context, options, symbols) {
        Ok(rust_code) => {
            println!("Generated generic Rust code:");
            println!("{}", rust_code);
            /* Output includes generic parameters like:
            pub fn process_data(
                name: impl Into<PyObject>,
                values: impl Into<PyObject>,
                args: impl IntoIterator<Item = impl Into<PyObject>>,
                kwargs: impl IntoIterator<Item = (impl AsRef<str>, impl Into<PyObject>)>
            ) -> impl Into<PyObject> { ... }
            */
        }
        Err(e) => {
            println!("Code generation failed: {}", e);
        }
    }
    
    Ok(())
}
```

### Working with AST Nodes

```rust
use python_ast::{parse, StatementType, ExprType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ast = parse("x = [1, 2, 3]", "list.py")?;
    
    match &ast.raw.body[0].statement {
        StatementType::Assign(assign) => {
            println!("Assignment to: {:?}", assign.targets);
            match &assign.value {
                ExprType::List(elements) => {
                    println!("List with {} elements", elements.len());
                }
                _ => {}
            }
        }
        _ => {}
    }
    
    Ok(())
}
```

## 📄 License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## 🔗 Links

- **Repository**: https://github.com/rexlunae/python-ast-rs
- **Documentation**: https://docs.rs/python-ast
- **Crate**: https://crates.io/crates/python-ast
- **Issues**: https://github.com/rexlunae/python-ast-rs/issues

---

**Note**: This library is experimental and under active development. While basic parsing works reliably, advanced features and code generation should be used with caution in production environments.