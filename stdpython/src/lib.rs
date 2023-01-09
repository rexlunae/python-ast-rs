//#![feature(c_variadic)]
//use std::collections::HashMap;
use std::fmt::Display;

pub fn print<S: Display>(s: S) {
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
