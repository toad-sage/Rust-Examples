use boxer::{Cons, CustomSmartPointer, List, Nil};
use std::ops::Deref;

#[derive(Debug)]
struct Iden {
    name: String,
}

fn main() {
    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
    );

    // let x = 5;
    // let y = MyBox::new(5);
    // let z = Iden {
    //     name: String::from("Riiti"),
    // };
    // let y = MyBox::new(z);
    // println!("{}", x);
    // println!("{:?}", *y);
    // printer(list);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers Created");
}

fn printer(b: List) {
    match b {
        Cons(val, li) => {
            println!("{}", val);
            printer(*li);
        }
        Nil => {
            return;
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
