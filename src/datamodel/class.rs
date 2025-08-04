//! traits for Python classes Objects. Classes are Object factories, meaning they create instances of objects.

use crate::Object;

/// The Class trait is used to define Python classes. Classes are Object factories, meaning they create instances of objects.
pub trait Class: Object + Default {
    type Super: Class + Into<Self>;

    /// Returns the method resolution order of the class.
    //fn mro(&self) -> Box<dyn Iterator<Item = Box<impl Class>>>;

    /// __new__ returns an instance of the class. Because of Rust's requirement that all objects be initialized,
    /// we require that the Object implement the Default trait. By default, we just return the default instance of the object.
    fn __new__() -> Self {
        Self::Super::__new__().into()
    }

    /// __init__ is called after __new__ and is used to initialize the object.
    fn __init__<A>(&mut self, _args: A) {}
}
