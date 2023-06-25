use pyo3::{PyAny, FromPyObject, PyResult};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use crate::tree::{Arg};
use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};

use log::debug;

#[derive(Clone, Debug, FromPyObject)]
pub struct Name {
    id: String,
}


#[derive(Clone, Debug, FromPyObject)]
pub struct Call {
    func: Name,
    args: Vec<Arg>,
    keywords: Vec<String>,
}

// This is just a way of extracting type information from Pyo3. And its a horrible hack.
#[derive(Clone, Debug, FromPyObject)]
struct GenericExpr {
    pub __doc__: String,
}

/* Expr(
    value=Call(
        func=Name(id='print', ctx=Load()),
        args=[],
        keywords=[]
    )
 ) */
#[derive(Clone, Debug, FromPyObject)]
pub enum ExprType {
    /*BoolOp(),
    NamedExpr(),
    BinOp(),
    UnaryOp(),
    Lambda(),
    IfExp(),
    Dict(),
    Set(),
    ListComp(),
    SetComp(),
    DictComp(),
    GeneratorExp(),
    Await(),
    Yield(),
    YieldFrom(),
    Compare(),*/
    Call(Call),
    /*FormattedValue(),
    JoinedStr(),
    Constant(),
    Attribute(),
    Subscript(),
    Starred(),
    Name(),
    List(),
    Tuple(),
    Slice(),*/

    Unimplemented(String),
}

/// An Expr only contains a single value key, which leads to the actual expression,
/// which is one of several types.
#[derive(Clone, Debug, FromPyObject)]
pub struct Expr {
    pub value: ExprType,
}


impl CodeGen for Expr {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        match self.value {
            ExprType::Call(call) => {
                let name = format_ident!("{}", call.func.id);
                let mut arg_stream = proc_macro2::TokenStream::new();

                for s in call.args.iter() {
                    arg_stream.extend(s.clone().to_rust(ctx)?);
                }
                Ok(quote!{#name(#arg_stream)})
            },
            //Expr::Break => Ok(quote!{break;}),
            _ => {
                let error = CodeGenError(format!("Expr not implemented {:?}", self), None);
                Err(Box::new(error))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    /*
    #[test]
    fn check_pass_expression() {
        let expression = Expr::Pass;
        let mut ctx = PythonContext::default();
        let tokens = expression.clone().to_rust(&mut ctx);

        println!("expression: {:?}, tokens: {:?}", expression, tokens);
        assert_eq!(tokens.unwrap().is_empty(), true);
    }

    #[test]
    fn check_break_expression() {
        let expression = Expr::Break;
        let mut ctx = PythonContext::default();
        let tokens = expression.clone().to_rust(&mut ctx);

        println!("expression: {:?}, tokens: {:?}", expression, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }

    #[test]
    fn check_continue_expression() {
        let expression = Expr::Continue;
        let mut ctx = PythonContext::default();
        let tokens = expression.clone().to_rust(&mut ctx);

        println!("expression: {:?}, tokens: {:?}", expression, tokens);
        assert_eq!(tokens.unwrap().is_empty(), false);
    }
    */

}
