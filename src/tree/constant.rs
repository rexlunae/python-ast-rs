//use std::fmt::Display;
use pyo3::{FromPyObject, PyAny, PyResult};
use crate::{CodeGen, PythonContext};
use proc_macro2::{TokenStream};
use quote::{format_ident, quote};

#[derive(Clone, Debug, Default, PartialEq)]
//#[pyo3(transparent)]
pub struct Constant(pub String);

impl<'a> FromPyObject<'a> for Constant {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        println!("Constant: {:?} {:?} {:?}", ob, crate::ast_dump(ob, Some(4))?, ob.getattr("value")?);
        let v: String = ob.getattr("value")?.extract()?;
        return Ok(Constant(v))
    }
}

impl CodeGen for Constant {
    fn to_rust(self, _ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let v = format_ident!("{}", self.0);
        Ok(quote!(#v))
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;
    //use super::*;

    #[test]
    fn signed_constant() {

    }

}