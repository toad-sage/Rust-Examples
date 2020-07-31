pub fn iterate(s: &str) {
    for c in s.chars() {
        println!("{}", c);
    }
}

pub fn string_operations() {
    let mut s = "initial contents".to_string();
    let d = String::from("initial contents");
    s += " is doomed";
    s.push_str("bar");
    println!("{}{}", s, d);
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    s.push('l');
    let c = 'd';
    s.push(c);
    println!("s is {}", s);

    // contacatenating two strings
    let a1 = String::from("hello, ");
    let d = a1.clone();
    let a2 = String::from("world");
    let s3 = a1 + &a2; // here a1 cannot be furthur used
    println!("{},{},{}", s3, d, a2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // Rust doesn't support string indexing eg:= s[3] is invalid

    // to avoid ownership being taken we can use format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("{} {} {} {}", s1, s2, s3, &s1[1..2]);
    let hello = "Здравствуйте"; // in this every character is 2 bytes long
    println!("{}->{}", &hello, &hello.len());
    let i = 0;
    println!("{}", &hello[i..4]); // first four bytes, so in this case 2 letters
    iterate(hello);
}
