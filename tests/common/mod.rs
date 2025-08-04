use python_ast::*;

/// Test utility functions for the python-ast library tests.

/// Asserts that parsing the given code succeeds and returns a module.
pub fn assert_parse_success(code: &str, filename: &str) -> Module {
    match parse(code, filename) {
        Ok(module) => {
            assert!(module.filename.is_some());
            assert_eq!(module.filename.as_ref().unwrap(), filename);
            module
        },
        Err(e) => panic!("Expected parsing to succeed, but got error: {}", e),
    }
}

/// Asserts that parsing the given code fails.
pub fn assert_parse_failure(code: &str, filename: &str) {
    match parse(code, filename) {
        Ok(_) => panic!("Expected parsing to fail, but it succeeded"),
        Err(_) => (), // Expected failure
    }
}

/// Creates a test Python code string with the given statements.
pub fn create_test_code(statements: &[&str]) -> String {
    statements.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_parse_success() {
        let code = "x = 1";
        let module = assert_parse_success(code, "test.py");
        
        assert_eq!(module.raw.body.len(), 1);
        assert_eq!(module.filename.as_ref().unwrap(), "test.py");
    }

    #[test]
    fn test_assert_parse_failure() {
        let invalid_code = "def broken_function(";
        assert_parse_failure(invalid_code, "broken.py");
    }

    #[test]
    fn test_create_test_code() {
        let statements = vec!["x = 1", "y = 2", "print(x + y)"];
        let code = create_test_code(&statements);
        
        let expected = "x = 1\ny = 2\nprint(x + y)";
        assert_eq!(code, expected);
    }
}