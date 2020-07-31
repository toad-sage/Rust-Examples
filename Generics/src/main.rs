struct Point<T> {
    x: T,
    y: T,
}

// impl<T> T will be of Type as in Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct SecondPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> SecondPoint<T, U> {
    fn mixup<V, W>(self, other: SecondPoint<V, W>) -> SecondPoint<T, W> {
        SecondPoint {
            x: self.x,
            y: other.y,
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("{}", largest_number(&number_list));

    // let char_list:Vec<char> = Vec::new();
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest_char(&char_list));

    let integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 2.0 };

    println!("{}", integer.x());
    // :: is for path to that function
    // . is used for calling an implementation function using object created
    println!("{}", Point::distance_from_origin(&_float));
    println!("{}", _float.distance_from_origin());

    let _integer_float = SecondPoint { x: 5, y: 4.0 };

    let p1 = SecondPoint { x: 5, y: 10.4 };
    let p2 = SecondPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    // or
    // let p3 = SecondPoint::mixup(p1, p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let number_list = vec![34, 50, 25, 100, 65];
    let a = String::from("Hi");
    let b = String::from("Hello");
    let c = String::from("World");
    let d = String::from("Amerigo");
    let e = String::from("Lamour");
    let slice_list = vec!["Hi", "World", "Labout", "Amerigo", "Lamour"];
    // println!("{}", largest(&slice_list));
    let string_list = vec![a, b, c, d, e];
    println!("{}", large(&string_list));
}

// &[i32] and Vec<i32> both will work
fn largest_number(v: &[i32]) -> i32 {
    let mut largest = v[0];
    for n in v {
        if largest < *n {
            largest = *n;
        }
    }
    largest
}

fn largest_char(v: &[char]) -> char {
    let mut largest = v[0];
    for c in v {
        if largest < *c {
            largest = *c;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Partial refers to the Trait,
// Trait defines the Type of variable that implements that trait
fn large<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
