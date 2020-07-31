use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     // lifetime of result should be lesser than x, y params of longest function
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// LifeTime Ellipsion Rules

// The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

// The third rule is if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
// This third rule makes methods much nicer to read and write because fewer symbols are necessary.

impl ImportantExcerpt<'_> {
    fn next(&self) -> &str {
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
