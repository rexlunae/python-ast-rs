# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`python-ast-rs` is a Rust library that provides Python AST (Abstract Syntax Tree) parsing and code generation capabilities. The library uses Python's own `ast` module via PyO3 to parse Python code, ensuring compatibility with the reference implementation. It includes experimental features for transpiling Python code to Rust.

**Key Features:**
- Parse Python code into Rust AST structures that mirror Python's AST
- Comprehensive support for expressions, statements, functions, classes, and control flow
- Rich documentation extraction and conversion from Python docstrings to Rust docs
- Dump AST structures for debugging using Python's `ast.dump()` function
- Experimental Python-to-Rust code generation (highly unstable but improving)
- Symbol table and scope analysis
- Extensible architecture with traits and macros

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
- **`traits.rs`** - Common traits for AST operations and PyO3 integration
- **`macros.rs`** - Macros for reducing boilerplate code
- **`parser_utils.rs`** - Generic utilities for parsing Python AST objects

### Key Dependencies
- **PyO3** - Python-Rust interop (version 0.25)
- **serde** - Serialization for converting Python objects to Rust
- **syn/quote/proc-macro2** - Rust code generation utilities
- **thiserror** - Error handling (version 2.0.12)

### AST Structure
The AST nodes in `src/ast/tree/` correspond to Python AST node types:

**Core Infrastructure:**
- `expression.rs` - Expression nodes and ExprType enum
- `statement.rs` - Statement nodes and StatementType enum  
- `module.rs` - Module-level constructs
- `node.rs` - Base Node trait and utilities

**Expression Types:**
- `lambda.rs` - Lambda expressions
- `if_exp.rs` - Conditional expressions (ternary operator)
- `dict.rs` - Dictionary literals
- `set.rs` - Set literals
- `tuple.rs` - Tuple literals
- `subscript.rs` - Subscript operations (indexing)
- `bin_ops.rs` - Binary operators
- `bool_ops.rs` - Boolean operators
- `unary_op.rs` - Unary operators
- `compare.rs` - Comparison operations
- `call.rs` - Function calls
- `constant.rs` - Literal constants
- `name.rs` - Variable names
- `attribute.rs` - Attribute access
- `list.rs` - List literals

**Statement Types:**
- `function_def.rs` - Function definitions
- `class_def.rs` - Class definitions
- `if_stmt.rs` - If statements
- `for_stmt.rs` - For loops
- `while_stmt.rs` - While loops
- `assign.rs` - Assignment statements
- `import.rs` - Import statements

**Support Structures:**
- `arguments.rs` - Function arguments and parameters
- `parameters.rs` - Parameter lists
- `keyword.rs` - Keyword arguments

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