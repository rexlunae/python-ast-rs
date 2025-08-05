# Async Runtime Configuration

The `python-ast-rs` crate now supports configurable async runtimes for Python async code generation. This allows you to choose which Rust async runtime to use when transpiling Python async functions to Rust.

## Supported Runtimes

### Built-in Runtimes

1. **Tokio** (default) - The most popular Rust async runtime
2. **async-std** - An alternative async runtime with std-like APIs  
3. **smol** - A small and fast async runtime

### Custom Runtimes

You can also specify a custom runtime by providing the attribute and import strings.

## Usage Examples

### Basic Usage

```rust
use python_ast::{parse_enhanced, CodeGenContext, PythonOptions, SymbolTableScopes, CodeGen, AsyncRuntime};

let python_code = r#"
import asyncio

async def fetch_data():
    await asyncio.sleep(1)
    return "data"

if __name__ == "__main__":
    result = asyncio.run(fetch_data())
    print(result)
"#;

// Using default (tokio)
let options = PythonOptions::default();

// Using async-std
let options = PythonOptions::with_async_std();

// Using smol
let options = PythonOptions::with_smol();

// Generate Rust code
let ast = parse_enhanced(python_code, "example.py")?;
let ctx = CodeGenContext::Module("example".to_string());
let symbols = SymbolTableScopes::new();
let rust_code = ast.to_rust(ctx, options, symbols)?;
```

### Convenience Methods

```rust
// Create options with specific runtime
let options = PythonOptions::with_tokio();      // Explicit tokio
let options = PythonOptions::with_async_std();  // async-std
let options = PythonOptions::with_smol();       // smol

// Custom runtime
let options = PythonOptions::with_custom_runtime(
    "my_runtime::main",  // attribute
    "my_runtime"         // import
);

// Programmatic configuration
let mut options = PythonOptions::default();
options.set_async_runtime(AsyncRuntime::AsyncStd);
```

### Custom Runtime Configuration

```rust
use python_ast::{PythonOptions, AsyncRuntime};

let options = PythonOptions::with_custom_runtime(
    "embassy_executor::main",  // The attribute for main function
    "embassy_executor"         // The crate to import
);

// Or using the enum directly
let mut options = PythonOptions::default();
options.async_runtime = AsyncRuntime::Custom {
    attribute: "embassy_executor::main".to_string(),
    import: "embassy_executor".to_string(),
};
```

## Generated Code Examples

### Input Python Code

```python
import asyncio

async def fetch_data():
    await asyncio.sleep(1)
    return "data"

async def main():
    result = await fetch_data()
    print(result)

if __name__ == "__main__":
    asyncio.run(main())
```

### Generated Rust Code (Tokio)

```rust
use stdpython::*;
use tokio;

pub async fn fetch_data() {
    // ... function body
}

pub async fn python_main() {
    // ... function body
}

#[tokio::main]
async fn main() {
    asyncio.run(python_main())
}
```

### Generated Rust Code (async-std)

```rust
use stdpython::*;
use async_std;

pub async fn fetch_data() {
    // ... function body
}

pub async fn python_main() {
    // ... function body
}

#[async_std::main]
async fn main() {
    asyncio.run(python_main())
}
```

### Generated Rust Code (Custom)

```rust
use stdpython::*;
use my_runtime;

pub async fn fetch_data() {
    // ... function body
}

pub async fn python_main() {
    // ... function body
}

#[my_runtime::main]
async fn main() {
    asyncio.run(python_main())
}
```

## Key Features

### Automatic Detection

The system automatically detects when Python async functions are present and:

1. **Adds runtime import**: `use tokio;`, `use async_std;`, etc.
2. **Generates async main**: `#[runtime::main] async fn main()`  
3. **Handles name conflicts**: Renames Python `main()` to `python_main()` if needed
4. **Consolidates execution**: Combines multiple `if __name__ == "__main__"` blocks

### Non-Async Code

For Python code without async functions, the system generates regular synchronous Rust code regardless of the configured async runtime:

```rust
use stdpython::*;

pub fn hello() {
    print("Hello, World!");
}

fn main() {
    hello()
}
```

## AsyncRuntime API

```rust
pub enum AsyncRuntime {
    Tokio,
    AsyncStd, 
    Smol,
    Custom { attribute: String, import: String },
}

impl AsyncRuntime {
    pub fn main_attribute(&self) -> &str { /* ... */ }
    pub fn import(&self) -> &str { /* ... */ }
}
```

## Migration from Previous Versions

Previous versions always used tokio. To maintain the same behavior:

```rust
// Old code (implicit tokio)
let options = PythonOptions::default();

// New code (explicit tokio, same behavior)  
let options = PythonOptions::with_tokio(); // or just default()
```

The default behavior remains unchanged - tokio is still the default runtime.

## Dependencies

Make sure to add the chosen async runtime to your `Cargo.toml`:

```toml
# For tokio
[dependencies]
tokio = { version = "1.0", features = ["full"] }

# For async-std
[dependencies] 
async-std = { version = "1.0", features = ["attributes"] }

# For smol
[dependencies]
smol = "2.0"
```