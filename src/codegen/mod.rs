use proc_macro2::TokenStream;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::default::Default;
//use std::env::SplitPaths;
use std::collections::{BTreeMap, HashSet};
use std::borrow::Borrow;
use std::path::{Path, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use std::str::Split;
use std::fs::File;
use std::io::prelude::*;

use crate::{sys_path, Scope};

#[derive(Debug)]
pub struct CodeGenError(pub String, pub Option<TokenStream>);
impl Error for CodeGenError {}

pub(crate) type Result<T> = std::result::Result<T, CodeGenError>;


impl Display for CodeGenError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Code generation failed.")
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
    pub fn search_path<S: Into<String> + Clone + Ord + Borrow<S>>(&self, file: S) -> Result<String> {
        for entry in self.python_path.clone() {
            let path_string = format!("{}{}{}", entry, MAIN_SEPARATOR, file.clone().into());
            if Path::new(path_string.as_str()).exists() {
                return Ok(path_string)
            }
        }
        Err(CodeGenError(String::from("Not found"), None))
    }

    /// Searches the Python path for the module and returns its contents.
    pub fn load<S: Into<String> + Clone + Ord + Borrow<S>>(&self, module: S) -> std::io::Result<String> {
        let module_string:String = module.into();
        let module_parts: Vec<&str> = module_string.split('.').collect();
        let module_path = if module_parts.len() == 1 {
            self.search_path(format!("{}.py", module_parts[0]))?
        } else {
            let first = self.search_path(module_parts[0]);
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

pub trait CodeGen {
    fn to_rust(self, ctx: &mut PythonContext) -> Result<TokenStream>;
}

