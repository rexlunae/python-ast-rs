use crate::tree::Arg;
use crate::codegen::{CodeGen, CodeGenError, PythonContext, Result};
use proc_macro2::TokenStream;
use crate::tree::statement::Statement;

use std::default::Default;


use quote::{format_ident, quote};
use pyo3::{PyAny, FromPyObject, PyResult};
use log::{debug, trace};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq)]
pub struct Parameter {
    pub arg: String,
}

impl CodeGen for Parameter {
    fn to_rust(self, _ctx: &mut PythonContext) -> Result<TokenStream> {
        let ident = format_ident!("{}", self.arg);
        Ok(quote!{
            #ident: PyAny
        })
    }
}
/// The parameter list of a function.
#[derive(Clone, Debug, Default, PartialEq, FromPyObject)]
pub struct ParameterList {
    pub posonlyargs: Vec<Parameter>,
    pub args: Vec<Parameter>,
    pub vararg: Option<Parameter>,
    pub kwonlyargs: Vec<Parameter>,

    //pub kw_defaults: Vec<Arg>,
    //pub kw_defaults: Option<Vec<Arg>>,

    pub kwarg: Option<Parameter>,

    //pub defaults: Vec<Arg>,
    pub defaults: Option<Vec<Arg>>,
}

/*
impl<'source> FromPyObject<'source> for ParameterList {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        let kwarg = ob.getattr("kwarg")?;
        debug!("==========----=====-----=========\nob.kwarg: {:#?}", kwarg);
        let arg = kwarg.getattr("arg")?;
        debug!(".arg: {:#?}", arg);
        Ok(ParameterList{
            ..Default::default()
        })
    }
}
*/

impl CodeGen for ParameterList {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream> {
        let mut stream = TokenStream::new();
        debug!("parameters: {:#?}", self);

        for arg in self.args {
            stream.extend(arg.clone().to_rust(ctx)?);
            stream.extend(quote!(,));
        }

        if let Some(arg) = self.vararg {
            let name = format_ident!("{}", arg.arg);
            stream.extend(quote!(#name: Vec<PyAny>));
            stream.extend(quote!(,));
        }

/*
        if let Some(arg) = self.kwarg {
            let name = format_ident!("{}", arg.arg);
            debug!("parsing: {:#?}", arg.arg);
            stream.extend(quote!(#name: PyDict<PyAny>));
            stream.extend(quote!(,));
        }
*/
        Ok(quote!(#stream))
    }
}

use crate::{parse};
use crate::tree::Module;

fn setup(input: &str) -> PyResult<Module> {
    let ast = parse(&input, "__test__")?;
    debug!("ast: {:#?}", ast);
    Ok(ast)
}

// It's fairly easy to break the automatic parsing of parameter structs, so we need to have fairly sophisticated
// test coverage for the various types of
#[cfg(test)]
mod tests {
    use test_log::test;
    use super::*;

    #[test]
    fn no_parameters() {
        let test_function = "def foo():\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 0)
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn one_parameter() {
        let test_function = "def foo1(a):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 1)
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn multiple_positional_parameter() {
        let test_function = "def foo2(a, b, c):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 3)
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn vararg_only() {
        let test_function = "def foo3(*a):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 0);
            assert_eq!(f.args.vararg, Some(Parameter{ arg: "a".to_string()}));
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn positional_and_vararg() {
        let test_function = "def foo4(a, *b):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(f.args.vararg, Some(Parameter{ arg: "b".to_string()}));
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn positional_and_vararg_and_kw() {
        let test_function = "def foo5(a, *b, c=7):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(f.args.vararg, Some(Parameter{ arg: "b".to_string()}));
            assert_eq!(f.args.kwonlyargs, vec![Parameter{ arg: "c".to_string()}]);
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }


    //XXX - This must pass to be Python-compatible.
    #[test]
    fn positional_and_kw() {
        let test_function = "def foo6(a, c=7):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 1);
            //assert_eq!(f.args.vararg, None);
            assert_eq!(f.args.kwonlyargs, vec![Parameter{ arg: "c".to_string()}]);
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn default_only() {
        let test_function = "def foo7(a=7):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(f.args.vararg, Some(Parameter{ arg: "b".to_string()}));
            assert_eq!(f.args.kwonlyargs, vec![Parameter{ arg: "c".to_string()}]);
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn kwargs_only() {
        let test_function = "def foo8(**a):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 0);
            assert_eq!(f.args.kwarg, Some(Parameter{ arg: "a".to_string()}));
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

    #[test]
    fn named_and_positional() {
        let test_function = "def foo9(a, *, b):\n    pass\n";
        let module = setup(test_function);

        let function_def_statement = module.unwrap().body[0].clone();
        debug!("statement: {:#?}", function_def_statement);

        if let Statement::FunctionDef(f) = function_def_statement {
            debug!("function definition: {:#?}", f);
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(f.args.vararg, None);
            assert_eq!(f.args.kwonlyargs, vec![Parameter{ arg: "b".to_string()}]);
        } else {
            panic!("Expected function definition, found {:#?}", function_def_statement);
        }
    }

}
