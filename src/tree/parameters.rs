use crate::tree::Arg;
use crate::codegen::{CodeGen, PythonContext};
use crate::ast_dump;

use proc_macro2::TokenStream;

use std::default::Default;


use quote::{format_ident, quote};
use pyo3::{FromPyObject};
use log::{debug};

#[derive(Clone, Debug, Default, FromPyObject, PartialEq)]
pub struct Parameter {
    pub arg: String,
}

impl CodeGen for Parameter {
    fn to_rust(self, _ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let ident = format_ident!("{}", self.arg);
        Ok(quote!{
            #ident: PyAny
        })
    }
}
/// The parameter list of a function.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParameterList {
    pub posonlyargs: Vec<Parameter>,
    pub args: Vec<Parameter>,
    pub vararg: Option<Parameter>,
    pub kwonlyargs: Vec<Parameter>,
    pub kw_defaults: Vec<Arg>,
    pub kwarg: Option<Parameter>,
    pub defaults: Vec<Arg>,
}

use pyo3::{PyAny, PyResult};

// We have to manually implement the conversion of ParameterList objects
// because under a number of conditions, the attributes are unset, which
// causes the derived trait to fail.

impl<'source> FromPyObject<'source> for ParameterList {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        println!("1: ob: {:#?}, {}", ob, ast_dump(&ob, Some(4))?);

        let posonlyargs = ob.getattr("posonlyargs")?;
        let posonlyargs_list: Vec<Parameter> = posonlyargs.extract()?;
        println!("2: ob.posonlyargs: {:?}, {:?}", posonlyargs, posonlyargs_list);

        //            "FunctionDef" => Ok(Statement::FunctionDef(FunctionDef::extract(ob)?)),
        let args = ob.getattr("args")?;
        let args_list: Vec<Parameter> = args.extract()?;
        println!("3: ob.args: {:?}, {:?}", args, args_list);

        let vararg = ob.getattr("vararg")?;
        let vararg_option: Option<Parameter> = vararg.extract()?;
        println!("4: ob.vararg: {:?} {:?} ", vararg, vararg_option);

        let kwonlyargs = ob.getattr("kwonlyargs")?;
        let kwonlyargs_list: Vec<Parameter> = kwonlyargs.extract()?;
        println!("5: ob.kwonlyargs: {:?}, {:?}", kwonlyargs, kwonlyargs_list);

        let kw_defaults = ob.getattr("kw_defaults")?;
        let kw_defaults_list: Vec<Arg> = kw_defaults.extract()?;
        println!("6: ob.kw_defaults: {:?} {:?}", kw_defaults, kw_defaults_list);

        let kwarg = ob.getattr("kwarg")?;
        let kwarg_option: Option<Parameter> = kwarg.extract()?;
        println!("7: ob.kwarg: {:?} {:?}", kwarg, kwarg_option);

        let defaults = ob.getattr("defaults")?;
        println!("8.1: ob.defaults: {:?}", defaults);
        println!("8.2: ob.defaults: {:?}, {}", defaults, ast_dump(defaults, Some(4))?);
        let defaults_list: Vec<Arg> = defaults.extract()?;
        println!("8.3: ob.defaults: {:?}, {:?}", defaults, defaults_list);

        Ok(ParameterList{
            posonlyargs: posonlyargs_list,
            args: args_list,
            vararg: vararg_option,
            kwonlyargs: kwonlyargs_list,
            kw_defaults: kw_defaults_list,
            kwarg: kwarg_option,
            defaults: defaults_list,

            ..Default::default()
        })
    }
}


impl CodeGen for ParameterList {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
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

// It's fairly easy to break the automatic parsing of parameter structs, so we need to have fairly sophisticated
// test coverage for the various types of
#[cfg(test)]
mod tests {
    use test_log::test;
    use super::*;

    use crate::{parse};
    use crate::tree::Module;
    use crate::tree::statement::Statement;
    use pyo3::{PyResult};



    fn setup(input: &str) -> PyResult<Module> {
        let ast = parse(&input, "__test__")?;
        debug!("ast: {:#?}", ast);
        Ok(ast)
    }

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

        println!("module: {:#?}", module);
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
            assert_eq!(f.args.defaults.len(), 1);
            assert_eq!(f.args.defaults[0], Arg::Constant(crate::Constant{ value: 7}));
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
