use proc_macro2::TokenStream;

pub trait CodeGen {
    fn to_rust<S>(input: S) -> TokenStream where S: Into<String>;
}