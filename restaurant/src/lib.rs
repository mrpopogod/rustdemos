mod front_of_house; // instead get this from another file with this filename

mod back_of_house {
    pub struct Breakfast {
        // as defined, you can specify your toast but you get the seasonal_fruit assigned
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"), // in summer you get peaches, in another season you get something else
            }
        }
    }

    // Because enums are usex exhaustively (e.g. match) either it's fully pub or fully private
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn serve_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path; current module is crate
    front_of_house::hosting::add_to_waitlist();

    //front_of_house::serving::take_order(); // doesn't work; serving is private to our scope since it's a child (whereas
    // with add_to_waitlist) it can access serving because it's in the ancestor scope, and then making take_order pub
    // completes the access

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist(); // note that the use directive below here still applies
}

// Taking advantage of use for our internal calls to reduce typing
// note: formatter seems to be self refs before crate refs, and concrete refs before module refs
use self::back_of_house::Appetizer; // the idiomatic way for use is functions should use up through the immediate parent module
                                    // while structs and enums are use all the way to the item (except when name conflict)
use self::front_of_house::serving; // relative paths are a thing
pub use crate::front_of_house::hosting; // so are absolute paths.  Here we also reexport with pub use which lets people call
                                        // hosting::add_to_waitlist() without needing to understand it's part of "front_of_house"

pub fn eat_at_restaurant_with_use() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    serving::take_order();

    let order = Appetizer::Soup;
}

use std::fmt;
use std::io;
use std::io::Result as IoResult;

// Example of how to handle name collisions
// Can either fully specify or use "as" to rename
fn function1() -> fmt::Result {
    fmt::Result::Err(fmt::Error)
}

fn function2() -> io::Result<()> {
    io::Result::Err(io::Error::from_raw_os_error(1))
}

fn function3() -> IoResult<()> {
    io::Result::Err(io::Error::from_raw_os_error(1))
}

// Example of nested paths to collapse use directives into a single line; this is the equivalent of:
// use std::cmp::Orering;
// use std::alloc;
use std::{cmp::Ordering, alloc};

// similarly, equivalent of
// use std::ascii;
// use std::ascii::EscapeDefault;
use std::ascii::{self, EscapeDefault};

// Glob is also a thing when you want everything
use std::collections::*;