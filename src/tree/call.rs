use pyo3::{FromPyObject, PyAny, PyResult};

use crate::{tree::Arg, Name};
use log::debug;

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct Call {
    pub func: Name,
    pub args: Vec<Arg>,
    pub keywords: Vec<String>,
}

/*
impl<'a> FromPyObject<'a> for Call {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        debug!("100000000 {}", crate::ast_dump(ob, Some(2))?);
        let func = ob.getattr("func")?;
        debug!("110000000 {}", crate::ast_dump(func, Some(2))?);
        let func_name = Name::extract(func)?;
        debug!("200000000");
        debug!("300000000");
        Err(pyo3::exceptions::PyValueError::new_err(format!("Unimplemented call {}...{}", ob, crate::ast_dump(ob, None)?)))
    }
}
*/