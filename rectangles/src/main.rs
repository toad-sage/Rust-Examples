use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // this is an associated block, which do not have any
    // self instance often used as constructor
    // It is called with ::sign , eg:= Rectangle::square(12)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// there can be more than one omplentation block
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut width = String::new();
    let mut height = String::new();

    io::stdin()
        .read_line(&mut width)
        .expect("Error in getting width");

    io::stdin()
        .read_line(&mut height)
        .expect("Error in getting height");

    let width: u32 = width.trim().parse().expect("Error in conversion");
    let height: u32 = height.trim().parse().expect("Error in conversion");

    let rect = Rectangle {
        width: width,
        height: height,
    };

    println!("The rect is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels by function.",
        area(&rect)
    );

    println!(
        "The area of the rectangle is {} square pixels by method.",
        rect.area()
    );

    println!(
        "The perimeter of the rectangle is {} pixels by method.",
        rect.perimeter()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let sq = Rectangle::square(12);

    println!("{}\n{:?}", inter(&rect, &rect2), sq);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn inter(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    rect1.can_hold(rect2)
}
