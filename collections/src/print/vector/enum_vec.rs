enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn creak() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in 0..row.len() {
        // let x = row.get(i);
        // println!("{:?}", x);
        match row.get(i) {
            Some(SpreadsheetCell::Int(x)) => println!("{}", x),
            Some(SpreadsheetCell::Text(x)) => println!("{}", x),
            Some(SpreadsheetCell::Float(x)) => println!("{}", x),
            None => println!("None"),
        }
    }}
