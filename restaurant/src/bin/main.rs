use ::hotel::hosting;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    hosting::add_to_waitlist();
    map.insert(1, 2);
}
