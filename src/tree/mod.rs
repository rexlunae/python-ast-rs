use std::default::Default;
use std::collections::HashMap;

use pyo3::{PyAny, FromPyObject, PyResult};

pub mod statement;
pub use statement::*;
use statement::Statement;

#[derive(Clone, Debug, Default, FromPyObject)]
pub struct Arg {
    pub name: String
}

#[derive(Clone, Debug, Default)]
pub struct Arguments {
    pub posonlyargs: Vec<Arg>,
    pub args: Vec<Arg>,
    pub vararg: Vec<Arg>,
    pub kwonlyargs: Vec<Arg>,
    pub kw_defaults: Vec<String>,
    pub kwarg: Arg,
    pub defaults: Vec<String>,
}

impl<'a> FromPyObject<'a> for Arguments {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        Ok(Self{
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug, FromPyObject)]
pub struct FunctionDef {
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Type {
    Unimplemented,
}

impl<'a> FromPyObject<'a> for Type {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        println!("Type: {:?}", ob);
        Ok(Type::Unimplemented)
    }
}

#[derive(Clone, Debug, FromPyObject)]
pub struct Module {
    pub body: Vec<Statement>,
    pub type_ignores: Vec<Type>,
}