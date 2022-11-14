extern crate proc_macro;

pub mod tree;
pub use tree::*;

pub mod codegen;
pub use codegen::*;

use pyo3::prelude::*;
use std::include_str;
use std::collections::HashMap;

/*
/// The direct Rust equivalent of the Python class of the same name,
/// albeit augmented with the token type text as a string.
#[derive(Clone, Debug, FromPyObject)]
pub struct TokenInfo {
    #[pyo3(attribute("type"))]
    pub token_type: usize,  /// type
    pub string: String, /// The token itself
    pub start: (usize,usize),  /// Start (line,col)
    pub end: (usize,usize),  /// End (line,col)
    pub line: String,
    pub token_text: String,
}
*/

/// Takes a string of bytes and returns the Python-tokenized version of it.
pub fn parse(input: &str, filename: &str) -> PyResult<tree::Module> {

    let pymodule_code = include_str!("parser.py");

    Python::with_gil(|py| -> PyResult<Module> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "parser.py", "parser")?;
        let t = pymodule.getattr("parse")?;
        assert!(t.is_callable());
        let args = (input, filename);

        let tree: tree::Module = t.call1(args)?.extract()?;
        
        Ok(tree)
    })
}

pub fn sys_path() -> PyResult<Vec<String>> {

    let pymodule_code = include_str!("path.py");

    Python::with_gil(|py| -> PyResult<Vec<String>> {
        let pymodule = PyModule::from_code(py, pymodule_code, "path.py", "path")?;
        let t = pymodule.getattr("path")?;
        assert!(t.is_callable());
        let args = ();
        let paths: Vec<String> = t.call1(args)?.extract()?;

        Ok(paths)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn check_token_stream() {
        let result = parse("#test comment
def foo():
    pass
", "test_case").unwrap();/*
        println!("tokens: {:?}", result);
        assert_eq!(result[0].token_text, "COMMENT");
        assert_eq!(result[1].token_text, "NL");
        assert_eq!(result[2].token_text, "NAME");
        assert_eq!(result[3].token_text, "NAME");
        assert_eq!(result[4].token_text, "OP");
        assert_eq!(result[5].token_text, "OP");
        assert_eq!(result[6].token_text, "OP");
        assert_eq!(result[7].token_text, "NEWLINE");
        assert_eq!(result[8].token_text, "INDENT");
        assert_eq!(result[9].token_text, "NAME");
        assert_eq!(result[10].token_text, "NEWLINE");
        assert_eq!(result[11].token_text, "DEDENT");
        assert_eq!(result[12].token_text, "ENDMARKER");*/
        println!("{:?}", result);
    }*/

}
