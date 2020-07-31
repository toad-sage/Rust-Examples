// importer1 is the default name of library which is package name
// can be changed using cargo.toml
use restaurant::{eat_at_restaurant, really_complicated_code};

fn main() {
    // println!("Using Library {}", importer1::really_complicated_code(2, 3));
    println!("Using Library {}", really_complicated_code(2, 3));

    // restaurant::eat_at_restaurant();
    //or
    eat_at_restaurant();
}
