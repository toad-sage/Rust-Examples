fn owner() {
    let mut s = String::from("hello");
    s.push_str(", world");

    // this will be termed as an error, as the ownership has changed from
    // s to a  and also string has not  copy trait, so printing s will be invalid
    // let a = s;
    // println!("{}", s);

    // let x = 10;
    // let y = x;
    // println!("{},{}", x, y);
    // takes_ownership(s);
    // here ownership has passed to takes_ownership() so variable s is not valid
    // println!("{}", s);

    /* This Works */
    // let x = 10;
    // makes_copy(x);
    // println!("{}", x);

    // after value is returned , again ownership comes to s
    // let (s, y) = takes_ownership(s);
    // println!("{},{}", s, y);
    // {
    //     let x = &mut s;
    //     takes_ownership(x);
    //     println!("{}", s);
    // };
    // let r1 = &s;
    // let r2 = &s;
    // {
    //     println!("{} {}", r1, r2);
    // }
    // let r3 = &mut s;
    // println!("{}", r3);

    let x = solver("helloRit");
    println!("{}", x);

    let mut s = String::from("Helllo.. Buffalo");
    let x = first_world(&s);
    // s.clear();
    println!("{} {}", x, s);

    let slice = &mut s[..2];
    println!("{}", slice);

    let word = first_word(&s);
    let d = &word[1..2];
    println!("{} {}", word, d);

    let x = solver(&s[..]); // String -> &str
    println!("{}", x);

    let a = [1, 2, 3, 4, 5];
    println!("{:?}", &a[2..4]);
}

fn _takes_ownership(some_string: &mut String) -> (String, String) {
    some_string.push_str(", funny");
    println!("{}", some_string);

    let mut y = some_string.clone();
    y.push_str(" revised");
    println!("{}", y);
    (some_string.to_string(), y)
}

fn _makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn solver(some_string: &str) -> &str {
    let t: Vec<&str> = some_string.split(" ").collect();
    t[0]
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}", bytes.iter().enumerate());
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
