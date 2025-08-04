# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`python-ast-rs` is a Rust library that provides Python AST (Abstract Syntax Tree) parsing and code generation capabilities. The library uses Python's own `ast` module via PyO3 to parse Python code, ensuring compatibility with the reference implementation. It includes experimental features for transpiling Python code to Rust.

**Key Features:**
- Parse Python code into Rust AST structures that mirror Python's AST
- Dump AST structures for debugging using Python's `ast.dump()` function
- Experimental Python-to-Rust code generation (highly unstable)
- Symbol table and scope analysis

## Development Commands

### Basic Development
```bash
# Build the project
cargo build

# Run type checking
cargo check

# Run all tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test

# Build and run with optimizations
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

## Architecture

The codebase is organized into several key modules:

### Core Modules
- **`ast/`** - Contains the AST data structures that parallel Python's AST
  - `tree/` - Individual AST node implementations (expressions, statements, etc.)
  - `node.rs` - Base AST node traits and utilities
  - `dump/` - AST dumping functionality using Python's ast.dump()

- **`parser/`** - Python code parsing using PyO3
  - Uses embedded Python code (`__init__.py`) to call Python's ast.parse()
  - Converts Python AST objects to Rust structures via serde

- **`codegen/`** - Experimental Python-to-Rust code generation
  - `python_options.rs` - Configuration for code generation
  - Uses `to_tokenstream` crate for generating Rust token streams

- **`symbols/`** - Symbol table and scope analysis
- **`scope.rs`** - Scope management utilities
- **`datamodel/`** - Python data model implementations (classes, namespaces, numbers)
- **`pytypes.rs`** - Python type system representations
- **`isidentifier/`** - Python identifier validation

### Key Dependencies
- **PyO3** - Python-Rust interop (version 0.21)
- **serde** - Serialization for converting Python objects to Rust
- **syn/quote/proc-macro2** - Rust code generation utilities
- **thiserror** - Error handling

### AST Structure
The AST nodes in `src/ast/tree/` correspond to Python AST node types:
- `expression.rs` - Expression nodes
- `statement.rs` - Statement nodes  
- `module.rs` - Module-level constructs
- `function_def.rs` - Function definitions
- Individual nodes for operators, literals, etc.

### Code Generation Flow
1. Parse Python code using `parser::parse()` which calls Python's ast.parse()
2. Convert Python AST to Rust structures via PyO3 extraction
3. Use `CodeGen` trait to generate Rust code (experimental)
4. Apply symbol table analysis and scope resolution

### Important Notes
- The project uses nightly Rust features (`#![feature(strict_provenance)]`)
- Code generation is experimental and highly unstable
- Python integration requires Python runtime via PyO3
- Many files contain debug println! statements that should typically be removed in production code