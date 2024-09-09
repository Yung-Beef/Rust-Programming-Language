mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonsal_fruit: String, // restaurant customer can choose their bread but the chef chooses the fruit
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonsal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer { // all variants are automatically public when you make an enum public
        Soup,
        Salad,
    }
}

// pub here makes this public so this line can be accessed by other code using this library
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // order a breakfast with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change your mind and switch to Wheat toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);
    //meal.seasonsal_fruit = String::from("blueberries"); // won't compile, can't see or modify fruit since it's private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // for an external crate
    //cratename::module

    // since hosting was brought into scope above with 'use'
    hosting::add_to_waitlist();
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {}
fn function2() -> io::Result<()> {}

// or

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}

//use std::cmp::Ordering;
//use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

use std::collections::*; // the glob operator, brings ALL public items defined in a path into scope
// often used when testing to bring in everything from the 'tests' module