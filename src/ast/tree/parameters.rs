use crate::Arguments;

/// ParameterList is now just an alias to Arguments for compatibility.
pub type ParameterList = Arguments;

// It's fairly easy to break the automatic parsing of parameter structs, so we need to have fairly sophisticated
// test coverage for the various types of
#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::parse;
    use crate::tree::statement::StatementType;
    use crate::tree::Module;
    use pyo3::PyResult;

    fn setup(input: &str) -> PyResult<Module> {
        let ast = parse(input, "__test__.py")?;
        Ok(ast)
    }

    #[test]
    fn no_parameters() {
        let test_function = "def foo():\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 0)
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn one_parameter() {
        let test_function = "def foo1(a):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 1)
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn multiple_positional_parameter() {
        let test_function = "def foo2(a, b, c):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 3)
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn vararg_only() {
        let test_function = "def foo3(*a):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 0);
            assert_eq!(
                f.args.vararg.as_ref().map(|p| &p.arg),
                Some(&"a".to_string())
            );
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn positional_and_vararg() {
        let test_function = "def foo4(a, *b):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(
                f.args.vararg.as_ref().map(|p| &p.arg),
                Some(&"b".to_string())
            );
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn positional_and_vararg_and_kw() {
        let test_function = "def foo5(a, *b, c=7):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(
                f.args.vararg.as_ref().map(|p| &p.arg),
                Some(&"b".to_string())
            );
            assert_eq!(f.args.kwonlyargs.len(), 1);
            assert_eq!(f.args.kwonlyargs[0].arg, "c".to_string());
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn positional_and_kw() {
        let test_function = "def foo6(a, c=7):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 2);
            assert_eq!(f.args.defaults.len(), 1);
            //assert_eq!(f.args.defaults[0], Arg::Constant(crate::Constant(Literal::parse(String::from("7")).unwrap())));
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn default_only() {
        let test_function = "def foo7(a=7):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(f.args.defaults.len(), 1);
            //assert_eq!(f.args.defaults[0], Arg::Constant(crate::Constant(Literal::parse(String::from("7")).unwrap())));
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn kwargs_only() {
        let test_function = "def foo8(**a):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 0);
            assert_eq!(
                f.args.kwarg.as_ref().map(|p| &p.arg),
                Some(&"a".to_string())
            );
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }

    #[test]
    fn named_and_positional() {
        let test_function = "def foo9(a, *, b):\n    pass\n";
        let module = setup(test_function).unwrap();

        let function_def_statement = module.raw.body[0].clone();

        if let StatementType::FunctionDef(f) = function_def_statement.statement {
            assert_eq!(f.args.args.len(), 1);
            assert_eq!(f.args.vararg, None);
            assert_eq!(f.args.kwonlyargs.len(), 1);
            assert_eq!(f.args.kwonlyargs[0].arg, "b".to_string());
        } else {
            panic!(
                "Expected function definition, found {:#?}",
                function_def_statement
            );
        }
    }
}
