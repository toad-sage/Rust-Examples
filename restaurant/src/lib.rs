mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("What's Up");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

// The
// front_of_house module isn’t public, but because the eat_at_restaurant
// function is defined in the same module as front_of_house (that is,
// eat_at_restaurant and front_of_house are siblings), we can refer to
// front_of_house from eat_at_restaurant

// if hosting is made public, we can use it's ancestor as well as hosting
// but not it's child

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peach"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting;

// use std::fmt::Result;
// use std::io::Result as IoResult;

pub fn eat_at_hotel() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
