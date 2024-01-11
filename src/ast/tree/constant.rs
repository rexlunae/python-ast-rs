use std::fmt::*;

use pyo3::{FromPyObject, PyAny, PyResult};
use proc_macro2::*;
use litrs::Literal;
use quote::{quote};
use log::debug;
use encoding::{Encoding, DecoderTrap, all::ISO_8859_6};

use crate::{
    Node, CodeGen, PythonOptions, CodeGenContext,
    SymbolTableScopes,
};

use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Clone, Debug, PartialEq)]
//#[pyo3(transparent)]
pub struct Constant(pub Option<Literal<String>>);

impl Serialize for Constant {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Constant {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let l = Literal::parse(s).expect("[3] Parsing the literal");
        Ok(Self(Some(l)))
    }
}

impl std::string::ToString for Constant {
    fn to_string(&self) -> String {
        match self.0.clone() {
            Some(c) => c.to_string(),
            None => "None".to_string(),
        }
    }
}

pub fn try_string(value: &PyAny) -> PyResult<Option<Literal<String>>> {
    let v: String = value.extract()?;
    let l = Literal::parse(format!("\"{}\"", v)).expect("[4] Parsing the literal");

    Ok(Some(l))
}

pub fn try_bytes(value: &PyAny) -> PyResult<Option<Literal<String>>> {
    let v: &[u8] = value.extract()?;
    let l = Literal::parse(format!("b\"{}\"", ISO_8859_6.decode(v, DecoderTrap::Replace).expect("decoding byte string"))).expect("[4] Parsing the literal");

    Ok(Some(l))
}

pub fn try_int(value: &PyAny) -> PyResult<Option<Literal<String>>> {
    let v: isize = value.extract()?;
    let l = Literal::parse(format!("{}", v)).expect("[4] Parsing the literal");

    Ok(Some(l))
}

pub fn try_float(value: &PyAny) -> PyResult<Option<Literal<String>>> {
    let v: f64 = value.extract()?;
    let l = Literal::parse(format!("{}", v)).expect("[4] Parsing the literal");

    Ok(Some(l))
}

pub fn try_bool(value: &PyAny) -> PyResult<Option<Literal<String>>> {
    let v: bool = value.extract()?;
    let l = Literal::parse(format!("{}", v)).expect("[4] Parsing the literal");

    Ok(Some(l))
}

// This will mostly be invoked when the input is None.
pub fn try_option(value: &PyAny) -> PyResult<Option<Literal<String>>> {
    let v: Option<&PyAny> = value.extract()?;
    debug!("extracted value {:?}", v);
    // If we got None as a constant, return None
    match v {
        None => Ok(None),
        // See if we can parse whatever we got that wasn't None.
        Some(_c) => {
            let l = Literal::parse(format!("{:?}", v)).expect("[5] Parsing the literal");
            Ok(Some(l))
        }
    }
}

// This is the fun bit of code that is responsible from converting from Python constants to Rust ones.
impl<'a> FromPyObject<'a> for Constant {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        // Extracts the values as a PyAny.
        let value = ob.getattr("value").expect(
            ob.error_message("<unknown>", "error getting constant value").as_str()
        );
        debug!("[2] constant value: {}", value);

        let l = if let Ok(l) = try_string(value) {
            l
        } else if let Ok(l) = try_bytes(value) {
            l
        // We have to evaluaet bool before int because if a bool is evaluated as it, it will be cooerced to an in.
        } else if let Ok(l) = try_bool(value) {
            l
        } else if let Ok(l) = try_float(value) {
            l
        } else if let Ok(l) = try_int(value) {
            l
        } else if let Ok(l) = try_option(value) {
            l
        } else {
            panic!("Failed to parse literal values {}", value);
        };

        Ok(Self(l))
    }
}

impl CodeGen for Constant {
    type Context = CodeGenContext;
    type Options = PythonOptions;
    type SymbolTable = SymbolTableScopes;

    fn to_rust(self, _ctx: Self::Context, _options: Self::Options, _symbols: Self::SymbolTable) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        match self.0 {
            Some(c) => {
                let v: TokenStream = c.to_string().parse()
                    .expect(format!("parsing Constant {}", c).as_str());
                Ok(quote!(#v))
            }
            None => Ok(quote!(None))
        }
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;
    //use super::*;
    use crate::{CodeGen, symbols::SymbolTableScopes};
    use log::debug;

    #[test]
    fn parse_string() {
        let s = crate::parse("'I ate a bug'", "test").unwrap();
        let ast = s.to_rust(crate::CodeGenContext::Module("test".to_string()), crate::PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        debug!("ast: {}", ast.to_string());

        assert_eq!("use stdpython :: * ; \"I ate a bug\"", ast.to_string());
    }

    #[test]
    fn parse_bytes() {
        let s = crate::parse("b'I ate a bug'", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module("test".to_string()), crate::PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; b\"I ate a bug\"", ast.to_string());
    }

    #[test]
    fn parse_number_int() {
        let s = crate::parse("871234234", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module("test".to_string()), crate::PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; 871234234", ast.to_string());
    }

    #[test]
    fn parse_number_neg_int() {
        let s = crate::parse("-871234234", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module("test".to_string()), crate::PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; - 871234234", ast.to_string());
    }

    #[test]
    fn parse_number_float() {
        let s = crate::parse("87123.4234", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module("test".to_string()), crate::PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; 87123.4234", ast.to_string());
    }

    #[test]
    fn parse_bool() {
        let s = crate::parse("True", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module("test".to_string()), crate::PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; true", ast.to_string());
    }

    #[test]
    fn parse_none() {
        let s = crate::parse("None", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module("test".to_string()), crate::PythonOptions::default(), SymbolTableScopes::new()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; None", ast.to_string());
    }
}