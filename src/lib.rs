#![feature(extend_one)]
#![feature(associated_type_bounds)]
extern crate proc_macro;

pub mod tree;
pub use tree::*;

pub mod codegen;
pub use codegen::*;

pub mod scope;
pub use scope::*;

use pyo3::prelude::*;
use std::include_str;



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
pub fn parse<'a>(input: &'a str, filename: &str) -> PyResult<tree::Module> {

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

/// Accepts any Python object and dumps it using the Python ast module.
pub fn ast_dump(o: &PyAny, indent: Option<u8>) -> PyResult<String> {

    let pymodule_code = include_str!("ast_dump.py");

    Python::with_gil(|py| -> PyResult<String> {
        // We want to call tokenize.tokenize from Python.
        let pymodule = PyModule::from_code(py, pymodule_code, "ast_dump.py", "parser")?;
        let t = pymodule.getattr("ast_dump")?;
        assert!(t.is_callable());
        let args = (o, indent);

        Ok(t.call1(args)?.extract()?)
    })

}

pub fn sys_path() -> PyResult<Vec<String>> {

    let pymodule_code = include_str!("path.py");

    Python::with_gil(|py| -> PyResult<Vec<String>> {
        let pymodule = PyModule::from_code(py, pymodule_code, "path.py", "path")?;
        let t = pymodule.getattr("path").expect("Reading path variable from interpretter");
        assert!(t.is_callable());
        let args = ();
        let paths: Vec<String> = t.call1(args)?.extract()?;

        Ok(paths)
    })
}

/*
/// Extracts the docstring from the top of any statement block.
pub fn extract_docstring(body: Vec<Statement>) -> (Option<String>, Vec<Statement>) {
    match body[0] {
        Statement::Expr(Expr{value: Constant(c)}) => (Some(c.value), Vec(body[1..])),
        _ => (None, body)
    }
}*/

#[cfg(test)]
mod tests {
    //use super::*;

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
        info!("{:?}", result);
    }*/

}
