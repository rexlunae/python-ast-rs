use proc_macro2::TokenStream;

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet},
    default::Default,
    error::Error,
    fmt::{Display, Formatter, Debug},
    fs::File,
    io::prelude::*,
    path::{Path, MAIN_SEPARATOR},
};

use crate::{sys_path, Scope};

#[derive(Debug)]
pub struct CodeGenError(pub String, pub Option<TokenStream>);
impl Error for CodeGenError {}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Code generation failed. {:#?}", self)
    }
}

/// The global context for Python compilation.
#[derive(Clone, Debug)]
pub struct PythonContext {
    /// Python imports are mapped into a given namespace that can be changed.
    pub python_namespace: String,

    /// The default path we will search for Python modules.
    pub python_path: Vec<String>,

    /// Collects all of the things we need to compile imports[module][asnames]
    pub imports: BTreeMap<String, HashSet<String>>,

    pub scope: Scope,

    pub stdpython: String,
    pub with_std_python: bool,

    pub allow_unsafe: bool,
}

impl Default for PythonContext {
    fn default() -> Self {
        Self {
            python_namespace: String::from("__python_namespace__"),
            // XXX: Remove unwrap.
            python_path: sys_path().unwrap(),
            imports: BTreeMap::new(),
            scope: Scope::default(),
            stdpython: "stdpython".to_string(),
            with_std_python: true,
            allow_unsafe: false,
        }
    }
}

impl PythonContext {

    /// Scans the Python path for the short name given, and returns the full path. Note that it only searches
    /// for the path itself, not any subpath.
    pub fn search_path<S: Into<String> + Clone + Ord + Borrow<S>>(&self, file: S) -> Result<String, Box<dyn std::error::Error>> {
        for entry in self.python_path.clone() {
            let path_string = format!("{}{}{}", entry, MAIN_SEPARATOR, file.clone().into());
            if Path::new(path_string.as_str()).exists() {
                return Ok(path_string)
            }
        }
        let error = CodeGenError(String::from("Not found"), None);
        Err(Box::new(error))
    }

    /// Searches the Python path for the module and returns its contents.
    pub fn load<S: Into<String> + Clone + Ord + Borrow<S>>(&self, module: S) -> Result<String, Box<dyn std::error::Error>> {
        let module_string:String = module.into();
        let module_parts: Vec<&str> = module_string.split('.').collect();
        let module_path = if module_parts.len() == 1 {
            self.search_path(format!("{}.py", module_parts[0]))?
        } else {
            let _first = self.search_path(module_parts[0]);
            format!("{}.py", module_parts[1..].join(format!("{}", MAIN_SEPARATOR).as_str()))
        };

        let mut file = File::open(&module_path)?;
        let mut s = String::new();

        file.read_to_string(&mut s)?;
        Ok(s)
    }


    pub fn import<S: Into<String> + Clone + Ord + Borrow<S>>(&mut self, from: S, to: S) {
        let f: String = from.into();
        let t: String = to.into();

        if !self.imports.contains_key(&f.clone()) {
            self.imports.insert(f.clone(), HashSet::new());
        }
        if let Some(m) = self.imports.get_mut(&f.clone()) {
            m.insert(t);
        }
    }
}

/// A trait for an object that can be converted to Rust code. Implemented generally be AST elements.
pub trait CodeGen: Debug {
    /// A trait method to input Rust code in a general sense. The output should be syntactical Rust,
    /// but may not be executable depending on
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>>;

    /// Only implemented by AST elements that can be compiled inside a trait. Others will generate
    /// an error.
    fn to_rust_trait_member(&self, _ctx: &mut PythonContext) -> Result<TokenStream, Box<dyn std::error::Error>> {
        Err(Box::new(CodeGenError(format!("Unsupported trait member: {:#?}", &self), None)))
    }
}
