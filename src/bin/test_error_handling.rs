use python_ast::{parse_enhanced, SourceLocation, Error};

fn main() {
    println!("Testing improved error handling...\n");

    // Test 1: Empty input (should succeed - empty files are valid in Python)
    println!("1. Testing empty input:");
    match parse_enhanced("", "empty.py") {
        Ok(_) => println!("   Successfully parsed empty file (as expected)"),
        Err(e) => println!("   Unexpected error: {}\n", e),
    }

    // Test 2: Invalid syntax
    println!("2. Testing invalid syntax:");
    match parse_enhanced("def broken(\n  pass", "syntax_error.py") {
        Ok(_) => println!("   Unexpectedly succeeded"),
        Err(e) => println!("   Error: {}\n", e),
    }

    // Test 3: Indentation error
    println!("3. Testing indentation error:");
    match parse_enhanced("def test():\npass", "indent_error.py") {
        Ok(_) => println!("   Unexpectedly succeeded"),
        Err(e) => println!("   Error: {}\n", e),
    }

    // Test 4: Valid code (should succeed)
    println!("4. Testing valid code:");
    match parse_enhanced("x = 1 + 2", "valid.py") {
        Ok(_) => println!("   Successfully parsed!"),
        Err(e) => println!("   Unexpected error: {}", e),
    }

    // Test 5: Test SourceLocation display
    println!("\n5. Testing SourceLocation formatting:");
    let loc1 = SourceLocation::new("test.py");
    println!("   Basic: {}", loc1);

    let loc2 = SourceLocation::with_position("test.py", Some(10), Some(5));
    println!("   With position: {}", loc2);

    let loc3 = SourceLocation::with_span("test.py", Some(10), Some(5), Some(12), Some(8));
    println!("   With span: {}", loc3);
}