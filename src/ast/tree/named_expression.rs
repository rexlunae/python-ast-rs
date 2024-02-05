use pyo3::FromPyObject;
use proc_macro2::TokenStream;
use quote::quote;
use serde::{Serialize, Deserialize};

use crate::{
    ExprType,
    CodeGen, PythonOptions, CodeGenContext,
    SymbolTableScopes,
};

/// A keyword argument, gnerally used in function calls.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NamedExpr {
    left: Box<ExprType>,
    right: Box<ExprType>,
}

impl<'a> FromPyObject<'a> for NamedExpr {
    fn extract(ob: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        let left = ob.getattr("left")?.extract::<ExprType>()?;
        let right = ob.getattr("right")?.extract::<ExprType>()?;
        Ok(NamedExpr {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

impl CodeGen for NamedExpr {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, ctx: Self::Context, options: Self::Options, symbols: Self::SymbolTable) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let left = self.left.clone().to_rust(ctx.clone(), options.clone(), symbols.clone()).expect(format!("parsing left side of named expression {:?}", self.left).as_str());
        let right = self.right.clone().to_rust(ctx, options, symbols).expect(format!("parsing right side of named expression {:?}", self.right).as_str());
        Ok(quote!(#left = #right))
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ExprType, Name, Constant};
    use litrs::*;

    #[test]
    fn test_named_expression() {
        let named_expression = NamedExpr {
            left: Box::new(ExprType::Name(Name {
                id: "a".to_string(),
            })),
            right: Box::new(ExprType::Constant(Constant(Some(Literal::Integer(IntegerLit::parse("1".to_string()).unwrap()))))),
        };
        let rust = named_expression.to_rust(CodeGenContext::Module("test".to_string()), PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        assert_eq!(rust.to_string(), "a = 1");
    }
}