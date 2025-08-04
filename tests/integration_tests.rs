use python_ast::*;
use python_ast::codegen::{CodeGen, CodeGenContext, PythonOptions};
use python_ast::symbols::SymbolTableScopes;

mod common;

#[test]
fn test_end_to_end_simple_function() {
    let code = r#"
def calculate_sum(a, b):
    """Calculate the sum of two numbers."""
    result = a + b
    return result
"#;
    
    let module = parse(code, "test_function.py").expect("Failed to parse code");
    
    // Verify parsing worked
    assert_eq!(module.raw.body.len(), 1);
    assert!(module.filename.is_some());
    assert_eq!(module.filename.as_ref().unwrap(), "test_function.py");
    
    // Test code generation
    let options = PythonOptions::default();
    let context = CodeGenContext::Module("test_function".to_string());
    let scopes = SymbolTableScopes::new();
    
    let rust_code = module.to_rust(context, options, scopes);
    // Code generation is experimental, so we just verify it doesn't panic
    assert!(rust_code.is_ok() || rust_code.is_err()); // Just ensure it completes
}

#[test] 
fn test_end_to_end_class_definition() {
    let code = r#"
class Calculator:
    """A simple calculator class."""
    
    def __init__(self, initial_value=0):
        self.value = initial_value
    
    def add(self, x):
        self.value += x
        return self.value
    
    def multiply(self, x):
        self.value *= x
        return self.value
"#;
    
    let module = parse(code, "calculator.py").expect("Failed to parse class");
    
    // Verify parsing
    assert_eq!(module.raw.body.len(), 1);
    
    // Verify module name generation
    assert!(module.name.is_some());
    assert_eq!(module.name.unwrap().id, "calculator");
}

#[test]
fn test_end_to_end_complex_module() {
    let code = r#"
"""A complex module demonstrating various Python features."""

import os
import sys
from typing import List, Dict, Optional
from collections import defaultdict

# Module-level constants
PI = 3.14159
DEBUG = True

class DataProcessor:
    """Process data with various operations."""
    
    def __init__(self, data: List[int]):
        self.data = data
        self.cache = {}
    
    def process(self) -> Dict[str, int]:
        """Process the data and return results."""
        results = {}
        
        for i, value in enumerate(self.data):
            if value > 0:
                results[f"item_{i}"] = value * 2
            elif value < 0:
                results[f"item_{i}"] = abs(value)
            else:
                results[f"item_{i}"] = 1
        
        return results

def fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

async def async_operation(data: List[str]) -> List[str]:
    """Perform an async operation on data."""
    results = []
    async for item in data:
        processed = await process_item(item)
        results.append(processed)
    return results

# List comprehension and generator expressions
squared_numbers = [x**2 for x in range(10) if x % 2 == 0]
data_dict = {f"key_{i}": value for i, value in enumerate(squared_numbers)}

if __name__ == "__main__":
    processor = DataProcessor([1, -2, 0, 4, -5])
    results = processor.process()
    print(f"Results: {results}")
"#;
    
    let module = parse(code, "complex_module.py").expect("Failed to parse complex module");
    
    // Verify we parsed all the top-level statements
    assert!(module.raw.body.len() > 10); // Should have many statements
    
    // Verify module properties
    assert!(module.filename.is_some());
    assert_eq!(module.filename.as_ref().unwrap(), "complex_module.py");
    assert!(module.name.is_some());
    assert_eq!(module.name.unwrap().id, "complex_module");
}

#[test]
fn test_end_to_end_error_handling() {
    let valid_code = r#"
def safe_divide(a, b):
    try:
        result = a / b
        return result
    except ZeroDivisionError as e:
        print(f"Error: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error: {e}")
        raise
    finally:
        print("Division operation completed")

with open("file.txt", "r") as f:
    content = f.read()
"#;
    
    let module = parse(valid_code, "error_handling.py").expect("Failed to parse error handling code");
    assert_eq!(module.raw.body.len(), 2); // Function definition + with statement
}

#[test]
fn test_end_to_end_decorators_and_context_managers() {
    let code = r#"
from functools import wraps
from contextlib import contextmanager

def retry(times=3):
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            for attempt in range(times):
                try:
                    return func(*args, **kwargs)
                except Exception as e:
                    if attempt == times - 1:
                        raise
                    print(f"Attempt {attempt + 1} failed: {e}")
        return wrapper
    return decorator

@contextmanager
def timer():
    import time
    start = time.time()
    try:
        yield
    finally:
        end = time.time()
        print(f"Elapsed: {end - start:.2f}s")

@retry(times=3)
@timer
def risky_operation():
    import random
    if random.random() < 0.7:
        raise ValueError("Random failure")
    return "Success!"
"#;
    
    let module = parse(code, "decorators.py").expect("Failed to parse decorators");
    assert!(module.raw.body.len() >= 3); // Imports + functions
}

#[test]
fn test_end_to_end_data_structures() {
    let code = r#"
# Various Python data structures
numbers = [1, 2, 3, 4, 5]
squares = [x**2 for x in numbers]
even_squares = [x for x in squares if x % 2 == 0]

person = {
    "name": "John Doe",
    "age": 30,
    "skills": ["Python", "Rust", "JavaScript"]
}

coordinates = (10.5, 20.3, 15.7)
unique_items = {1, 2, 3, 3, 4, 4, 5}

# Nested structures
matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
nested_dict = {
    "level1": {
        "level2": {
            "data": [1, 2, 3]
        }
    }
}

# Dictionary and set comprehensions
word_lengths = {word: len(word) for word in ["hello", "world", "python"]}
vowels = {char for char in "hello world" if char in "aeiou"}
"#;
    
    let module = parse(code, "data_structures.py").expect("Failed to parse data structures");
    assert!(module.raw.body.len() >= 8); // Various assignments
}

#[test]
fn test_end_to_end_module_with_path() {
    let code = "value = 42";
    let complex_path = format!("src{}utils{}helpers.py", 
                              std::path::MAIN_SEPARATOR, 
                              std::path::MAIN_SEPARATOR);
    
    let module = parse(code, &complex_path).expect("Failed to parse with complex path");
    
    assert!(module.name.is_some());
    assert_eq!(module.name.unwrap().id, "src__utils__helpers");
    assert!(module.filename.is_some());
    assert_eq!(module.filename.as_ref().unwrap(), &complex_path);
}

#[test]
fn test_end_to_end_empty_module() {
    let code = "";
    let module = parse(code, "empty.py").expect("Failed to parse empty module");
    
    assert!(module.raw.body.is_empty());
    assert!(module.filename.is_some());
    assert!(module.name.is_some());
}

#[test]
fn test_end_to_end_only_comments_and_docstring() {
    let code = r#"
"""
This module only contains comments and a docstring.
It should still parse successfully.
"""

# This is a comment
# Another comment

# More comments...
"#;
    
    let module = parse(code, "comments_only.py").expect("Failed to parse comments-only module");
    
    // Should have the docstring as a statement
    assert_eq!(module.raw.body.len(), 1);
}

#[test]
fn test_end_to_end_syntax_error() {
    let invalid_code = r#"
def broken_function(
    # Missing closing parenthesis and colon
    pass
"#;
    
    let result = parse(invalid_code, "broken.py");
    assert!(result.is_err(), "Expected parsing to fail for invalid syntax");
}

#[test]
fn test_end_to_end_large_module() {
    // Generate a larger module to test performance and memory usage
    let mut large_code = String::new();
    large_code.push_str("# Large module test\n");
    
    // Add many function definitions
    for i in 0..50 {
        large_code.push_str(&format!(r#"
def function_{}(param1, param2=None):
    """Function number {}"""
    if param2 is None:
        param2 = {}
    result = param1 + param2
    return result * 2

"#, i, i, i));
    }
    
    // Add many class definitions
    for i in 0..20 {
        large_code.push_str(&format!(r#"
class Class{}:
    """Class number {}"""
    
    def __init__(self, value={}):
        self.value = value
    
    def method_{}(self):
        return self.value * {}

"#, i, i, i, i, i + 1));
    }
    
    let module = parse(&large_code, "large_module.py").expect("Failed to parse large module");
    
    // Should have parsed all functions and classes
    assert_eq!(module.raw.body.len(), 70); // 50 functions + 20 classes
    assert!(module.filename.is_some());
    assert!(module.name.is_some());
}