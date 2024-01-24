use pyo3::{FromPyObject, PyAny, PyResult};
use proc_macro2::TokenStream;
use quote::{quote};
use serde::{Serialize, Deserialize};

use crate::{
    dump,
    Node,
    ExprType,
    CodeGen, PythonOptions, CodeGenContext, CodeGenError,
    SymbolTableScopes,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Compares {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,

    Unknown,
}


impl<'a> FromPyObject<'a> for Compares {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let err_msg = format!("Unimplemented unary op {}", dump(ob, None)?);
        Err(pyo3::exceptions::PyValueError::new_err(
            ob.error_message("<unknown>", err_msg)
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Compare {
    ops: Vec<Compares>,
    left: Box<ExprType>,
    comparators: Vec<ExprType>,
}

impl<'a> FromPyObject<'a> for Compare {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        log::debug!("ob: {}", dump(ob, None)?);

        // Python allows for multiple comparators, rust we only supports one, so we have to rewrite the comparison a little.
        let ops: Vec<&PyAny> = ob.getattr("ops")
            .expect(ob.error_message("<unknown>", "error getting unary operator").as_str())
            .extract().expect("getting ops from Compare");

        let mut op_list = Vec::new();

        for op in ops.iter() {
            let op_type = op.get_type().name().expect(
                ob.error_message("<unknown>", format!("extracting type name {:?} for binary operator", op)).as_str()
            );

            let op = match op_type {
                "Eq" => Compares::Eq,
                "NotEq" => Compares::NotEq,
                "Lt" => Compares::Lt,
                "LtE" => Compares::LtE,
                "Gt" => Compares::Gt,
                "GtE" => Compares::GtE,
                "Is" => Compares::Is,
                "IsNot" => Compares::IsNot,
                "In" => Compares::In,
                "NotIn" => Compares::NotIn,

                _ => {
                    log::debug!("Found unknown Compare {:?}", op);
                    Compares::Unknown
                }
            };
            op_list.push(op);
        }

        let left = ob.getattr("left")
            .expect(ob.error_message("<unknown>", "error getting comparator").as_str());

        let comparators = ob.getattr("comparators")
            .expect(ob.error_message("<unknown>", "error getting compoarator").as_str());
        log::debug!("left: {}, comparators: {}", dump(left, None)?, dump(comparators, None)?);


        let left = ExprType::extract(left).expect("getting binary operator operand");
        let comparators: Vec<ExprType> = comparators.extract().expect("getting comparators from Compare");

        log::debug!("left: {:?}, comparators: {:?}, op: {:?}", left, comparators, op_list);

        return Ok(Compare{
            ops: op_list,
            left: Box::new(left),
            comparators: comparators,
        });

    }
}

impl<'a> CodeGen for Compare {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut outer_ts = TokenStream::new();
        let left = self.left.clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
        let ops = self.ops.clone();
        let comparators = self.comparators.clone();

        let mut index = 0;
        for op in ops.iter() {
            let comparator = comparators.get(index).expect("getting comparator").clone().to_rust(ctx.clone(), options.clone(), symbols.clone())?;
            let tokens = match op {
                Compares::Eq => quote!(((#left) == (#comparator))),
                Compares::NotEq => quote!(((#left) != (#comparator))),
                Compares::Lt => quote!(((#left) < (#comparator))),
                Compares::LtE => quote!(((#left) <= (#comparator))),
                Compares::Gt => quote!(((#left) > (#comparator))),
                Compares::GtE => quote!(((#left) >= (#comparator))),
                Compares::Is => quote!((&(#left) == &(#comparator))),
                Compares::IsNot => quote!((&(#left) != &(#comparator))),
                Compares::In => quote!(((#comparator).get(#left) == Some(_))),
                Compares::NotIn => quote!(((#comparator).get(#left) == None)),

                _ => {
                    let error = CodeGenError::NotYetImplemented(format!("Compare not implemented {:?}", self));
                    return Err(error.into())
                }
            };

            index += 1;

            outer_ts.extend(tokens);
            if index < ops.len() {
                outer_ts.extend(quote!( && ));
            }
        }
        Ok(outer_ts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_simple_eq() {
        let options = PythonOptions::default();
        let result = crate::parse("1 == 2", "test_case").unwrap();
        log::info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(CodeGenContext::Module("test_case".to_string()), options, SymbolTableScopes::new());
        log::info!("module: {:?}", code);
    }

    #[test]
    fn test_complex_compare() {
        let options = PythonOptions::default();
        let result = crate::parse("1 < a > 6", "test_case").unwrap();
        log::info!("Python tree: {:?}", result);
        //info!("{}", result);

        let code = result.to_rust(CodeGenContext::Module("test_case".to_string()), options, SymbolTableScopes::new());
        log::info!("module: {:?}", code);
    }
}
