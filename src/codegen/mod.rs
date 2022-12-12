use proc_macro2::TokenStream;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::default::Default;
use std::env::SplitPaths;
use std::collections::{BTreeMap, HashSet};
use std::borrow::Borrow;
use std::path::{Path, MAIN_SEPARATOR};

use crate::sys_path;

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
    pub imports: BTreeMap<String, HashSet<String>>
}

impl Default for PythonContext {
    fn default() -> Self {
        Self {
            python_namespace: String::from("__python_namespace__"),
            // XXX: Remove unwrap.
            python_path: sys_path().unwrap(),
            imports: BTreeMap::new(),
        }
    }
}

impl PythonContext {

    /// Scans the Python path for the short name given, and returns the full path.
    pub fn search_path<S: Into<String> + Clone + Ord + Borrow<S>>(&self, file: S) -> Result<String> {
        for entry in self.python_path.clone() {
            let path_string = format!("{}{}{}", entry, MAIN_SEPARATOR, file.clone().into());
            if Path::new(path_string.as_str()).exists() {
                return Ok(path_string)
            }
        }
        Err(CodeGenError(String::from("Not found"), None))
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

