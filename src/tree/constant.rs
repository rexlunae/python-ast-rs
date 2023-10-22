use pyo3::{FromPyObject, PyAny, PyResult};
use crate::{CodeGen, PythonOptions, CodeGenContext};
use proc_macro2::*;
use litrs::Literal;
use quote::{quote};
use std::fmt::*;

use log::debug;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_6;

#[derive(Clone, Debug)]
//#[pyo3(transparent)]
pub struct Constant(pub Literal<String>);

pub fn try_string(value: &PyAny) -> PyResult<Literal<String>> {
    let v: String = value.extract()?;
    let l = Literal::parse(format!("\"{}\"", v)).expect("[4] Parsing the literal");

    Ok(l)
}

pub fn try_bytes(value: &PyAny) -> PyResult<Literal<String>> {
    let v: &[u8] = value.extract()?;
    let l = Literal::parse(format!("b\"{}\"", ISO_8859_6.decode(v, DecoderTrap::Replace).expect("decoding byte string"))).expect("[4] Parsing the literal");

    Ok(l)
}

pub fn try_int(value: &PyAny) -> PyResult<Literal<String>> {
    let v: isize = value.extract()?;
    let l = Literal::parse(format!("{}", v)).expect("[4] Parsing the literal");

    Ok(l)
}

pub fn try_float(value: &PyAny) -> PyResult<Literal<String>> {
    let v: f64 = value.extract()?;
    let l = Literal::parse(format!("{}", v)).expect("[4] Parsing the literal");

    Ok(l)
}

pub fn try_bool(value: &PyAny) -> PyResult<Literal<String>> {
    let v: bool = value.extract()?;
    let l = Literal::parse(format!("{}", v)).expect("[4] Parsing the literal");

    Ok(l)
}

// This is the fun bit of code that is responsible from converting from Python constants to Rust ones.
impl<'a> FromPyObject<'a> for Constant {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        // Extracts the values as a PyAny.
        let value = ob.getattr("value").expect("getting constant value");
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
        } else {
            panic!("Failed to parse literal values {}", value);
        };

        Ok(Self(l))
    }
}

impl CodeGen for Constant {
    type Context = CodeGenContext;
    type Options = PythonOptions;

    fn to_rust(self, _ctx: Self::Context, _options: Self::Options) -> std::result::Result<TokenStream, Box<dyn std::error::Error>> {
        let v: TokenStream = self.0.to_string().parse()
            .expect(format!("parsing Constant {}", self.0).as_str());
        Ok(quote!(#v))
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;
    //use super::*;
    use crate::CodeGen;
    use log::debug;

    #[test]
    fn parse_string() {
        let s = crate::parse("'I ate a bug'", "test").unwrap();
        let ast = s.to_rust(crate::CodeGenContext::Module, crate::PythonOptions::default()).unwrap();
        debug!("ast: {}", ast.to_string());

        assert_eq!("use stdpython :: * ; \"I ate a bug\"", ast.to_string());
    }

    #[test]
    fn parse_bytes() {
        let s = crate::parse("b'I ate a bug'", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module, crate::PythonOptions::default()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; b\"I ate a bug\"", ast.to_string());
    }

    #[test]
    fn parse_number_int() {
        let s = crate::parse("871234234", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module, crate::PythonOptions::default()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; 871234234", ast.to_string());
    }

    #[test]
    fn parse_number_neg_int() {
        let s = crate::parse("-871234234", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module, crate::PythonOptions::default()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; -871234234", ast.to_string());
    }

    #[test]
    fn parse_number_float() {
        let s = crate::parse("87123.4234", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module, crate::PythonOptions::default()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; 87123.4234", ast.to_string());
    }

    #[test]
    fn parse_bool() {
        let s = crate::parse("True", "test").unwrap();
        println!("parsed value: {:?}", s);
        let ast = s.to_rust(crate::CodeGenContext::Module, crate::PythonOptions::default()).unwrap();
        println!("ast: {:?}", ast);

        assert_eq!("use stdpython :: * ; true", ast.to_string());
    }
}