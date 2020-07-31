mod print;
// use print::string;
// use print::vector;
use std::collections::HashMap;

fn main() {
    // vector::vector_operation();
    // string::string_operations();
    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    // or

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using
    // them

    let score = scores1.get("Blue");
    println!("{:?} {}", score, scores1["Blue"]);

    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }

    scores.insert(String::from("Blue"), scores["Blue"] + 12);

    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // or_insert method provides a mutable refence of new inserted data
        *count += 1;
    }

    println!("{:?}", map);

    let v = vec![4, 2, 3, 4, 5];
    println!("{}", mean(&v));
    println!("{}", mode(&v));
    println!("{}", median(&v));
    let new_string = convert(String::from("hello"));
    println!("{}", new_string);

    let mut map = HashMap::new();
    add_employee(&mut map, String::from("Add Sally to Sales"));
    add_employee(&mut map, String::from("Add Alex to Engineering"));
    add_employee(&mut map, String::from("Add Robert to Sales"));
    add_employee(&mut map, String::from("Add Rachel to Engineering"));
    add_employee(&mut map, String::from("Add Rubina to Sales"));

    let v = &map[&String::from("Engineering")];
    for x in v {
        print!("{} ", x);
    }
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut cnt = 0.0;
    for i in v {
        cnt += *i as f64;
    }
    cnt / (v.len() as f64)
}

fn median(v: &Vec<i32>) -> i32 {
    let mut x = v.clone();
    x.sort();
    let pos: i32 = v.len() as i32 / 2;
    x[pos as usize]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut val: i32 = 0;
    let mut ans: i32 = v[0];
    for i in v {
        let cnt = map.entry(i).or_insert(0);
        *cnt += 1;
        if val < *cnt {
            val = *cnt;
            ans = *i;
        }
    }
    ans
}

fn convert(s: String) -> String {
    // let char_array = s.chars();
    // println!("{:?}", char_array);
    let x = &s[0..1];
    let y = &s[1..];
    let mut n = String::from(y);
    n.push_str(x);

    n
}

fn add_employee(h: &mut HashMap<String, Vec<String>>, s: String) {
    let sentence: Vec<&str> = s.split_whitespace().collect();
    let v = h.entry(String::from(sentence[3])).or_insert(vec![]);
    v.push(String::from(sentence[1]));
}
