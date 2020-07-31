pub fn print1(val: &i32) {
    println!("{}", val);
}

pub fn print2(v: &Vec<i32>, index: usize) {
    match v.get(index) {
        Some(third) => print1(third),
        None => println!("There is no third Element"),
    }
}
