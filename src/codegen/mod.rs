use proc_macro2::TokenStream;

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CodeGenError(pub String, pub Option<TokenStream>);
impl Error for CodeGenError {}

pub(crate) type Result<T> = std::result::Result<T, CodeGenError>;


impl Display for CodeGenError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Code generation failed.")
    }
}

pub trait CodeGen {
    fn to_rust(self) -> Result<TokenStream>;
}