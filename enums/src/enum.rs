enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrI {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// just as struct, we can also implement methods on enum

impl Message {
    fn call(&self) {
        // method should be define here
    }
}

fn enum() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddrI::V6(String::from("::1"));
    println!("{:?}", loopback);

    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));

    match loopback {
        IpAddr2::V6(val) => println!("{}", val),
        _ => println!("Too much Fin!"),
    }

    let m = Message::Write(String::from("Hi"));
    m.call();

    // Option is inbuilt enum for handling null values
    let n: Option<i32> = None;
    let some_number = Some(5);

    match some_number {
        Some(val) => println!("{}", val),
        _ => println!("Sing Hossana"),
    }
}
