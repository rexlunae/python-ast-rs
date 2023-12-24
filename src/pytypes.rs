use std::{
    collections::VecDeque,
    iter::Iterator,
};

/// The Python list type. Generally, it's used with a `dyn Object` trait object to allow
/// for arbitrary contents. Since Rust doesn't have a native type that supports mixed type
/// lists, we need to invent something.
pub type List<T> = VecDeque<Box<T>>;

/// An interface for any data structure that works like a Python List.
pub trait ListLike<T> {
    fn append(&mut self, x: T);
    fn insert(&mut self, i: usize, x: T);

    fn extend(&mut self, iterable: Box<dyn Iterator<Item = T>>) {
        for item in iterable {
            self.append(item);
        }
    }

    //fn remove(&mut self, x: Box<T>);
}

impl<T: Iterator> ListLike<T> for List<T> {
    fn append(&mut self, x: T) {
        self.push_back(Box::new(x));
    }

    fn insert(&mut self, i: usize, x: T) {
        VecDeque::insert(self, i, Box::new(x))
    }

    // An important diffference between the Pythonic List and the Rust VecDeque is that
    // VecDeque.remove() takes an index into the array, whereas Python List.remove() takes
    // a *value*, searches the list for it, and removes the first instance of that.
    /*fn remove(&mut self, x: Box<T>) {
        match self.iter().position(|&i| i.into_inner() == x.into_inner()) {
            Some(p) => VecDeque::remove(self, p).or_else(0),
            _ => return,
        };
    }*/
}
