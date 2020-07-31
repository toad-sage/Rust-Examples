fn vars() {
    let x = 5;
    println!("The value of x is: {}", x);

    // x = 6;
    // println!("The value of x is: {}", x);
    // you cannot assign
    // twice to immutable variable x , because you tried to assign a second value to
    // the immutable x variable.

    // below code is correct

    let mut y = 4;
    println!("The value of y is: {}", y);
    y = 5;
    println!("The value of y is: {}", y);

    const MAX_POINTS: u32 = 100_00;
    println!("Maximum point is: {}", MAX_POINTS);

    // shadowing
    let z = 5;
    let z = z + 1;
    println!("The current of value z is: {}", z);

    // diff between mut vs shadowing

    // using mut
    // let mut spaces = "    ";
    // this will be compiled as error , since a string type is getting integer
    // spaces = spaces.len();

    //using shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of space is: {}", spaces);

    let z: u32 = 1_00 + 2_00 + 10_00; // _ is not considered , just for visual aid
    println!("The value of z is: {}", z); // default type of integer is i32 and fastest to compile

    let x = 2.0; // default type is f64
    let y: f32 = 3.0; // f32
    println!("The value of y is {1} and x is {0}", x, y);

    // Numeric Operations

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    println!(
        "The different values are: {},{},{},{},{}",
        sum, difference, product, quotient, remainder
    );

    // Boolean Operations
    let t = true;
    let f: bool = false;
    let c = 'z';
    let z = 'Z';

    println!("Values: {},{},{},{}", t, f, c, z);

    // tuples are immutable in nature
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    // we cab destructure as well
    let (x, y, z) = _tup;
    // accessing value by index
    let one = _tup.2;
    println!("{},{},{},{},{}", _tup.1, x, y, z, one);

    // array
    let a = [1, 2, 3, 4];
    println!("{},{},{}", a[0], a[1], a[2]);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{0},{1},{0}", months[2], months[3]);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    //     if you want to create an array that contains the same value for each
    // element, you can specify the initial value, followed by a semicolon, and then the
    // length of the array in square brackets

    let mut a = [3; 5];
    println!("{},{},{},{},{}", a[0], a[1], a[2], a[3], a[4]);
    a[3] = 4;
    another_function();
}
