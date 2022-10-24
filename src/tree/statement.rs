use pyo3::{PyAny, FromPyObject, PyResult};

use crate::tree::FunctionDef;

// This is just a way of extracting type information from Pyo3.
#[derive(Clone, Debug, FromPyObject)]
struct GenericStatement {
    pub __doc__: String,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Break,
    Continue,
    Pass,
    Import(String),
    FunctionDef(FunctionDef),

    Unimplemented(String),
}

impl<'a> FromPyObject<'a> for Statement {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let gen_statement = GenericStatement::extract(ob)?;
        let parts: Vec<&str> = gen_statement.__doc__.split("(").collect();

        match parts[0] {
            "Pass" => Ok(Statement::Pass),
            "Continue" => Ok(Statement::Continue),
            "Break" => Ok(Statement::Break),
            "FunctionDef" => Ok(Statement::FunctionDef(FunctionDef::extract(ob)?)),
            _ => Ok(Statement::Unimplemented(String::from(parts[0]))),
        }

    }
}
