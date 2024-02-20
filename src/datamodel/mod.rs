//! Module representing the Python Data Model in Rust form.
//! See: [here](https://docs.python.org/3/reference/datamodel.html).

/// The Python Object. Anything that implements this trait is a Python Object.
pub trait Object: Sized {
    /// Returns the unique identifier of the object, which is the memory address of the object.
    fn id(&self) -> usize {
        std::ptr::addr_of!(*self) as usize
    }

    /// Returns the type name of the object.
    fn r#type(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }

    /// Returns the type name of the object.
    fn is<T: Object>(&self, other: &Option<T>) -> bool {
        self.id() == other.id()
    }

    /// __getattribute__ is called to look up an attribute of the object.
    fn __getattribute__(&self, name: impl AsRef<str>) -> Option<impl Object> {
        std::option::Option::<i32>::None
    }

    /// __setattribute__ is called to set an attribute of the object.
    fn __setattribute__<T: Object>(&mut self, name: impl AsRef<str>, value: T) {
        unimplemented!()
    }

    /// __delattribute__ is called to delete an attribute of the object.
    fn __delattribute__(&mut self, name: impl AsRef<str>) {
        unimplemented!()
    }

    /// __dir__ is called to list the attributes of the object.
    fn __dir__(&self) -> impl Iterator<Item = &String> {
        unimplemented!();
        Vec::<String>::new().iter()
    }
}

impl Object for i8 {}
impl Object for i16 {}
impl Object for i32 {}
impl Object for i64 {}
impl Object for i128 {}
impl Object for u8 {}
impl Object for u16 {}
impl Object for u32 {}
impl Object for u64 {}
impl Object for u128 {}
impl Object for String {}
impl Object for &str {}
impl Object for bool {}
impl Object for f32 {}
impl Object for f64 {}
impl Object for char {}

// There is a special implementation for the Option type, which allows Option<T>::None to be None in all cases.
impl<T: Object> Object for Option<T> {
    fn is<U: Object>(&self, other: &Option<U>) -> bool {
        match (self, other) {
            (Some(_), std::option::Option::None) => false,
            (std::option::Option::None, Some(_)) => false,
            (Some(a), Some(_b)) => a.is(other),
            (std::option::Option::None, std::option::Option::None) => true,
        }
    }
}

/// The Python None object.
#[allow(non_upper_case_globals)]
pub static None: Option<String> = std::option::Option::<String>::None;

/// The Python NotImplemented object.
#[allow(non_upper_case_globals)]
pub static NotImplemented: Option<&str> = Some("NotImplemented");

/// The Python NotImplemented object.
#[allow(non_upper_case_globals)]
pub static Ellipsis: &str = "...";

pub mod number;
pub use number::*;

pub mod namespace;
pub use namespace::*;

pub mod class;
pub use class::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let x = 5;
        let y = 6;
        println!("x: {:p}, type: {}", &x, x.r#type());
        assert_eq!(Object::id(&x), Object::id(&x));
        assert_ne!(Object::id(&x), Object::id(&y));
    }

    #[test]
    fn test_none() {
        let x = &None;
        let y: Option<i32> = std::option::Option::None;

        assert_eq!(x.is(&None), true);
        assert_ne!(x.is(&NotImplemented), true);
        assert_eq!(y.is(&None), true);
        assert_ne!(y.is(&NotImplemented), true);
    }
}
