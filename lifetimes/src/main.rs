fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", return_passed_in(result));

    let five = 5;
    ref_and_val(result, five);
    println!("Five was copied {}", five);
    let my_str = String::from("A string");
    ref_and_val(result, my_str);
    //println!("We lost my_str {}", my_str); // We gave away my_str

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("First sentence of Moby Dick is \"{}\"", i.announce_and_return_part("now hear this"));

    // The lifetime parameter on part means that this code won't compile; it can
    // check and see that my_str goes out of scope and then we try and reference
    // it again.  If we remove the println it compiles because even though ie
    // doesn't technically go out of scope until the closing curly brace it has
    // effectively gone out of scope because it isn't referenced again
    /* let ie: ImportantExcerpt;
    {
        let my_str = String::from("A string literal");
        ie = ImportantExcerpt { part: &my_str[..] };
    }

    println!("This won't work: {}", ie.part); */

    // Here's an example of static lifetimes.  String literals are in the code,
    // so they have an implicit static lifetime.  Otherwise you have to explicitly
    // give the variable a static lifetime.
    let s;
    {
        s = "A str"; 
    }

    println!("This is a static string {}", s);

    // Here's an example of how static is not a panacea; it only says you expect
    // the reference to last forever; the compiler will tell you if you're full
    // of shit (as we are here; my_str goes out of scope when main ends).  Unlike 
    // the example above where a reference has its backer go out of scope but isn't 
    // used again (when we remove the println) so the compiler is ok, with static 
    // we explicitly say to the compiler that this MUST live forever.  So it really 
    // should only ever applie to literals, as those are the only ones that will
    // be guaranteed to be valid for as long as the program executes (as string 
    // literals are in the code itself, so as long as the program is loaded the
    // literal is in memory)
    /* let my_str = String::from("Foo");
    let str_ref: &'static String = &my_str; */
}

// 'a ('ident is the syntax) defines a lifetime, and specifically the goal here
// is to show the relationship between the lifetime of all the variables involved
// because x or y could be returned then the return must have at least the lifetime
// of x and y, so we give them all the same lifetime identifier
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Naive implementation of longest that fails the lifetime checks; the compiler
// can't figure out if x or y is returned and thus can't figure out what the
// lifetime bounds need to be in general (e.g. if we explicitly returned x then y
// could go out of scope right after this is called while x is still in scope
// and the returned value would be valid).
/* fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

// More generally, as soon as there are multiple references and a returned 
// reference then we need to specify lifetime, even if you can exhaustively
// prove the lifetime of the resources involved, as in the below
/* fn return_first(x: &str, y: &str) -> &str {
    x
} */

// But with only a single parameter we're good; this is because a returned reference
// must come from one of the parameters (either the param itself or one of the param's
// children).  So if there's only one reference then obviously only that one param
// defines the lifetime.  As soon as there's a second param we're asking the compiler
// to deeply analyze our code to figure out all the possibilities, and good job, it's
// the halting problem.  So it only looks at the signature to see if it can calculate
// a concrete lifetime
fn return_passed_in(x: &str) -> &str {
    x
}

// Similarly, we can have extra non-ref values and lifetime is still good,
// as non-refs were either copied/cloned or given
fn ref_and_val<T>(x: &str, _y: T) -> &str {
    x
}

// Compiler figured out down here that we're returning a value that doesn't match
// the return; the return has a lifetime but y doesn't, so it's bad.  Simple type
// matching, where type includes the lifetime
/* fn bad_lifetime<'a>(x: &'a str, y: &str) -> &'a str {
    y
} */

// If you want a struct to hold a reference then you need to include lifetime information
// so the compiler can make sure you aren't fucking up.
struct ImportantExcerpt<'a> {
    part: &'a str,
}


impl<'a> ImportantExcerpt<'a> {
    // We add a method to ImportantExceprt.  The rules for auto applying lifetimes are to
    // first give each ref param a lifetime, then if there's only one give the return
    // the same lifetime, then if one of the params is self give the return self's
    // lifetime.  So this one could be rewritten as:
    // fn announce_and_return_part<'a, 'b>(&'a self, announcement: &'b str) -> &'a str
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    // This method won't compile because it has the same signature as the above, but
    // the attempted return is &'b str, when the signature says we'll return &'a str
    /* fn announce_and_return_announce(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        announcement
    } */
}