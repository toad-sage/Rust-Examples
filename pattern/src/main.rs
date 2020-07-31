enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let mut stack = vec![];

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    one();

    let point = (3, 5);
    print_coordinates(&point);

    // let x = Some(5);
    // let y = 10;
    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(y) => println!("Matched, y = {:?}", y),
    //     _ => println!("Default case, x = {:?}", x),
    // }
    // println!("at the end: x = {:?}, y = {:?}", x, y);

    // let x = 5;
    // match x {
    //     1..=5 => println!("one through five"),
    //     _ => println!("something else"),
    // }

    // let x = 'c';
    // match x {
    //     'a'..='j' => println!("early ASCII letter"),
    //     'k'..='z' => println!("late ASCII letter"),
    //     _ => println!("something else"),
    // }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn one() {
    let v = vec!['a', 'b', 'c', 'd'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
