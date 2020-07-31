use overloader::{Meters, Millimeters};
use std::ops::Add;
use std::ops::Deref;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        print!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 3));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// using NewType Pattern , we can apply external traits on external type,
// which is against the orphan rule

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

fn main() {
    let res = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!("{}", res);
    println!("{:?}", res.outline_print());

    let milli = Millimeters(10);
    let met = Meters(10);

    println!("{:?}", milli + met);

    println!("A baby dog is called a {}", Dog::baby_name()); // Spot
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // Puppy

    let w = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("p = {}", w.len());

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let closure = returns_closure();
    println!("{}", closure(4));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
