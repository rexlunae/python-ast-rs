use pyo3::{FromPyObject, PyAny, PyResult};
use crate::{CodeGen, PythonContext};
use proc_macro2::*;
use litrs::Literal;
use quote::{quote};
use std::fmt::*;

use log::debug;

#[derive(Clone, Debug)]
//#[pyo3(transparent)]
pub struct Constant(pub Literal<String>);

impl<'a> FromPyObject<'a> for Constant {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        debug!("constant type: {}...{}", ob.get_type().name()?, crate::ast_dump(ob, Some(8))?);
        debug!("constant value: {:?}", ob.getattr("value")?);
        let v: String = ob.getattr("value")?.extract()?;
        if let Ok(l) = Literal::parse(v) {
            Ok(Self(l))
        }
        else {
            Err(pyo3::PyErr::from_value(ob))
        }

    }
}

impl CodeGen for Constant {
    fn to_rust(self, _ctx: &mut PythonContext) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        Ok(quote!(self.0))
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