#[derive(PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub use crate::List::{Cons, Nil};

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
    }
}
