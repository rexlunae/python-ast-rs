use pyo3::{FromPyObject};

use crate::{tree::Arg, Name};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq)]
//#[pyo3(transparent)]
pub struct Call {
    pub func: Name,
    pub args: Vec<Arg>,
    pub keywords: Vec<String>,
}
