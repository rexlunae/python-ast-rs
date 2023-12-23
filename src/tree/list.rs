use pyo3::{FromPyObject, PyAny};
use quote::{quote};
use proc_macro2::TokenStream;

//use crate::tree::{Expr};
use crate::codegen::{CodeGen, PythonOptions, CodeGenContext};
use crate::symbols::SymbolTableScopes;

// There are two concepts of List in the same place here. There's the "List" type that represents a node from the Python AST,
// as received by the Rust AST converter, and there's the List representation of the Python List type. For the sake of
// consistency, we're using the same type as we use to model
pub type ListContents = crate::pytypes::List::<dyn CodeGen>;


#[derive(Clone, Debug, Default, FromPyObject)]
pub struct List<'a> {
    pub elts: Vec<&'a PyAny>,
    pub ctx: Option<String>,
}

impl<'a> CodeGen for List<'a> {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, _ctx: Self::Context, _options: Self::Options, _symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let ts = TokenStream::new();
        log::debug!("================self:{:#?}", self);
        for elt in self.elts {
            let el: &PyAny = elt.extract()?;
            log::debug!("elt: {}", crate::ast_dump(el, None)?);
            //ts.extend(elt.to_rust(ctx, options).expect("parsing list element"))
        }
        Ok(quote!(vec![#ts]))
    }
}

// It's fairly easy to break the automatic parsing of parameter structs, so we need to have fairly sophisticated
// test coverage for the various types of
#[cfg(test)]
mod tests {
    use test_log::test;
    use crate::StatementType;
    use crate::ExprType;
    use std::panic;

    #[test]
    fn parse_list() {
        let module = crate::parse("[1, 2, 3]", "nothing").unwrap();
        let statement = module.body[0].statement.clone();
        match statement {
            StatementType::Expr(e) => {
                match e.value {
                    ExprType::List(c) => {
                        log::debug!("{:#?}", c);
                        let list = c.0;
                        assert_eq!(list.len(), 3);
                        //assert_eq!(list[0].into_inner(), 1);
                        //assert_eq!(list[1].into_inner(), 2);
                        //assert_eq!(list[2].into_inner(), 3);
                    },
                    _ => panic!("Could not find inner expression")
                }
            }
            _ => panic!("Could not find outer expression.")
        }
    }
}
