use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

// MAke a type for condition checking
// suppose , we have to take a number which should always be b/w 1 and 100.
// an if condition will do checking for us. But another way is to make a setter and
// getter

struct Guess {
    value: i32,
}

impl Guess {
    /// value can only be between 1 and 100 , both including
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100");
        }
        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem Creating the File: {:?}", e),
            },
            other_error => panic!("Problem Opening the file: {:?}", other_error),
        },
    };

    println!("{:?}", f);

    // using closures

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem Creating the file: {:?}", error))
        } else {
            panic!("Problem Opening The File: {:?}", error);
        }
    });

    println!("{:?}", f);

    // unwrap is just a shortcut for invoking panic! macro,
    // when error is encountered
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hi.txt").expect("Failed to open hi.txt");

    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }

    match read_username_from_file2() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }

    match read_username_from_file3() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }

    match read_username_from_file4() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }

    let num = Guess::new(3);
    println!("The value of num is {}", num.value());
}

// propagating result or error , to the function calling it
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// or

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// or

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// or

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
