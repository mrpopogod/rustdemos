// When you define a module in a separate file you only are defining the body of the module that was declared in the
// original file, so no need for mod <filename> {}
pub mod hosting {
    pub fn add_to_waitlist() {
        super::serving::take_order() // relative path; current module is hosting, super == front_of_house
    } // rust privacy is strict; everything in module land is private by default and you can't see what's inside a
      // private child, though a child can see what is in its ancestors' siblings

    pub fn seat_at_table() {} // shows as a dead code warning because the root module isn't pub, so it can't be hit
                              // by people consuming the library
}

pub mod serving; // externalizing this module as well; since we're already in an external module we use directories to namespace
