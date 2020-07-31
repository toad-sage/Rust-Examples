use std::cmp::Ordering;

fn looper() {
    let z = fun(4, 5);
    let x = five();
    // functions, macros and {} are expressions ,that return value
    // since expression return value, the line which return value
    // should not be ending with a ; , if ; is put then ,
    // expression will turn into a statement thus will not
    // return any value
    let y = {
        let x = 3;
        x + 1
    };

    println!("THe value of y is: {} and value of x is {},{}", y, x, z);
    // while_loop()
    // fibo()
}

fn _conditionals() -> bool {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was somewhat true");
    } else {
        println!("condition was false");
    }

    let cond = true;
    let number = if cond { 5 } else { number + 2 };
    number < 5
}

fn _loop() {
    // for infinite loop till ctrl+c
    // loop {
    //     println!("again");
    // }
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result)
}

fn fibo() {
    const MAX_VAL: usize = 100_00;
    let mut a = [0; MAX_VAL];
    a[1] = 1;
    for i in 2..MAX_VAL {
        a[i] = (a[i - 1] + a[i - 2]) % 1000_000_007;
    }
    for i in 0..210 {
        println!("{}", a[i]);
    }
}

fn _while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    // we can use index in while loop to iterate array, but slow
    let mut index = 0;
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // use this instead if possible
    for elem in a.iter() {
        println!("the value is: {}", elem);
    }

    // use for loop for iterating in a range
    for number in (1..a.len()).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn _another_function() {
    println!("Another Function");
}
/// for returning 5
fn five() -> i32 {
    return 5;
    // or
    // 5
}

fn fun(x: i32, y: i32) -> i32 {
    println!("The value of sum is {}", x + y);
    // return x + y;
    // or
    // if x % 2 == 0 {
    // return x + y;
    // here we will have to use return keyword because if expects void
    // }

    let z = match x.cmp(&y) {
        Ordering::Equal => x,
        Ordering::Less => x - y,
        Ordering::Greater => x + y,
    };

    z
}
