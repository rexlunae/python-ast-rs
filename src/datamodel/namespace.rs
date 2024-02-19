//! Rust traits used to abstract Python data objects that are namespaces and contain symbols.
//! This includes Modules, Functions, Classes, and other objects.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::Name;

pub enum PyPath {
    Name(Name),
    SubModule(Vec<PyPath>),
    Super,
}

pub trait NameSpace: crate::Object {
    /// Returns the name of the object
    fn name(&self) -> Name;

    /// Returns the docstring, if any.
    fn doc(&self) -> Option<String>;

    /// Returns the namespace of the Object.
    fn dict<K, V>(&self) -> HashMap<K, V>;
}

#[cfg(test)]
mod tests {
    use super::*;
}
