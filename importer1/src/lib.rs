pub fn really_complicated_code(a: u8, b: u8) -> u8 {
    a + b
}

// it will search file with name front_of_house and will replace
// content with this file
mod front_of_house;

// In this case, the crate root �le
// is src/lib.rs, but this procedure also works with binary crates
// whose crate root �le is
// src/main.rs.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
