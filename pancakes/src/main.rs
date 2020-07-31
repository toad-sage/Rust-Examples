use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Cookies;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn main() {
    Pancakes::hello_macro();
    Cookies::hello_macro();
}
