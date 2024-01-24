//! Options for Python compilation.

use std::{
    collections::{BTreeMap, HashSet},
    default::Default,
};

use crate::Scope;
use pyo3::{prelude::*, PyResult};

pub fn sys_path() -> PyResult<Vec<String>> {

    let pymodule_code = include_str!("path.py");

    Python::with_gil(|py| -> PyResult<Vec<String>> {
        let pymodule = PyModule::from_code(py, pymodule_code, "path.py", "path")?;
        let t = pymodule.getattr("path").expect("Reading path variable from interpretter");
        assert!(t.is_callable());
        let args = ();
        let paths: Vec<String> = t.call1(args)?.extract()?;

        Ok(paths)
    })
}


/// The global context for Python compilation.
#[derive(Clone, Debug)]
pub struct PythonOptions {
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

impl Default for PythonOptions {
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
