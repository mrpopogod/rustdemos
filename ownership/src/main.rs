// Demonstration of the various ownership and reference concepts
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    //println!("Value of s is {}", s);  // if we uncomment this code it's a compile error

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    println!("Value of x is {}", x); // this is valid

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    println!("Value of s1 is {}", s1); // since we have ownership we can reference

    let s2 = String::from("hello"); // s2 comes into scope
    println!("Value of s2 at start is {}", s2);

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    //println!("Value of s2 is {}", s2); // Again, invalid because when it was given back it went into a different variable
    println!("Value of s3 is {}", s3); // Effectively we did let s3 = s2, with the resultant ownership transfer

    let s1 = String::from("hello");

    let (s1, len) = calculate_length_tuple(s1); // a cumbersome way to preserve s1, but it works

    println!("The length of '{}' is {}.", s1, len);

    let len = calculate_length(&s1); // instead we pass by reference; note that unlike C you need & on the caller and the destination

    println!("The length of '{}' is {}.", s1, len);

    let test = String::from("test");
    let mut reference = &test;
    println!(
        "Dereference dreference contains t: {}",
        (*reference).contains("t")
    ); // dereferencing is optional for member access
    println!("Reference access contains t: {}", reference.contains("t"));
    println!("Reference is pointing at {}", reference); // we can get the original value
    {
        let test2 = String::from("test2");
        reference = &test2;
        println!("The inner scope has reference pointing at {}", reference); // println! is smart enough to handle a reference passed in
    }

    //println!("Let's see if this works {}", *reference); // compile error because test2 doesn't live long enough
    reference = &test;
    println!("Back to normal with {}", reference); // this is fine, because we fix reference's binding before using it

    // Demonstration of restrictions on references, scoped for ease of use
    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            // let r2 = &mut s; // not cool
            println!("First mutable ref is {}", r1);
            let r2 = &mut s; // Here the compiler gets smart and sees that it can move the mut reference from r1 to r2, since we don't use r1
            println!("Second mutable ref is {}", r2);
            // println!("Is r1 still valid? {}", r1); // But if we uncomment this line, now r1 is valid at the same time r2 is and we violate the no two mut refs rule
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let r2 = &mut s;
        println!("Second mutable ref is {}", r2);
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &s;
            let r2 = &s;
            println!("We could get two immutable refs: {} and {}", r1, r2);
            /*let r3 = &s;
            let r4 = &mut s;
            println!("But this is not allowed: immute {} mut {}", r3, r4);*/ // This block demonstrates that you can't mix and match mutable and immutable in a single scope
        } // and then they go out of scope so we're free to

        let r3 = &mut s;
        println!("And the mutable ref: {}", r3);
    } // A key thing about references is they have much tighter scoping than variables; variables are scoped by control structure
      // but references are scoped by last usage
} // Everything goes out of scope.  Drop is called for resources that we still have ownership of and memory is freed.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("take_ownership says {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("makes_copy says {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// Uses a tuple so the string is not lost by the caller
fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// Uses pass by reference so the string is not lost by the caller
fn calculate_length(s: &String) -> usize {
    s.len()
}
