use proc_macro2::TokenStream;
use pyo3::{FromPyObject, PyErr};
use quote::{format_ident, quote};

use crate::{CodeGen, CodeGenContext, IsIdentifier, PythonOptions, SymbolTableScopes};

use serde::{Deserialize, Serialize};

/// Identifiers represent valid Python identifiers.
#[derive(Clone, Debug, Default, Eq, FromPyObject, Hash, PartialEq, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Identifier(String);

impl TryFrom<&str> for Identifier {
    type Error = PyErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.isidentifier()? {
            Ok(Identifier(value.to_string()))
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyNameError, _>(format!(
                "Invalid Identifier: {}",
                String::from(value)
            )))
        }
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.0
    }
}

/// Names are Python identifiers, separated by '.'
#[derive(Clone, Debug, Default, Eq, FromPyObject, Hash, PartialEq, Serialize, Deserialize)]
pub struct Name {
    pub id: String,
}

impl TryFrom<&str> for Name {
    type Error = PyErr;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split('.');
        log::debug!("parts: {:?}", parts);

        let mut v = Vec::new();
        for part in parts {
            let ident = Identifier::try_from(part)?;
            v.push(String::from(ident.as_ref()));
        }

        Ok(Name { id: v.join(".") })
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.id.as_str()
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.id
    }
}

impl CodeGen for Name {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(
        self,
        _ctx: Self::Context,
        _options: Self::Options,
        _symbols: Self::SymbolTable,
    ) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // Handle dotted names (like "os.path") by converting them to Rust module paths
        if self.id.contains('.') {
            let parts: Vec<&str> = self.id.split('.').collect();
            let idents: Vec<_> = parts.iter().map(|part| format_ident!("{}", part)).collect();
            Ok(quote!(#(#idents)::*))
        } else {
            let name = format_ident!("{}", self.id);
            Ok(quote!(#name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_name_works() {
        let name = Name::try_from("this.symbol");
        assert!(name.is_ok());
    }

    #[test]
    fn bad_name_works() {
        let name = Name::try_from("this.0symbol");
        assert!(name.is_err());
    }
}
