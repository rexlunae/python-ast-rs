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
}

impl Object for i32 {}
impl Object for String {}
impl Object for &str {}
impl Object for bool {}
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
