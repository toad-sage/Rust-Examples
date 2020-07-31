#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    NewYork,
    PaloAlto,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The Quarter is from state {:?}", state);
            20
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1),
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::California);
    println!("{}", value_in_cents(&coin));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}\n{:?}", six, none);

    if let Some(i) = five {
        println!("{}", i);
    } else {
        println!("None");
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
