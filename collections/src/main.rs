use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let third_score = 30;
    scores.insert(String::from("Green"), third_score);

    println!("Copyable is copied into map, so I can still acccess original: {}", third_score);

    println!("Let's dump the map");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut reference_scores = HashMap::new();
    reference_scores.insert(String::from("Green"), &10);
    let int_ref = 20;
    reference_scores.insert(String::from("Red"), &int_ref);

    println!("int_ref is still in scope, so I still can use it: {}", int_ref);

    let mut another_map = HashMap::new();
    {
        let _third_int = 4;
        another_map.insert(String::from("bluh"), &4); // this works; since it's a literal it's inside the code memory an the reference is valid forever
        //fourth_map.insert(String::from("blah"), &_third_int);  // this doesn't work; it detects that third_int goes out of scope
    }

    match another_map.get(&String::from("bluh")) {
        Some(i) => println!("We still have the literal: {}", i),
        None => println!("LIteral poofed"),
    };

    match another_map.get(&String::from("blah")) {
        Some(i) => println!("We still have third_int: {}", i),
        None => println!("Third_int poofed"),
    };

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // At this point ownership of these non-Copy values is taken
    // println!("Oh no, someone stole my values: {} {}", field_name, field_value); // which means this is a compile error

    // Here's something slick around building a map from two vectors (could also do collect on a vector of k,v tuples)
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores: HashMap<_, _> = // need to specify the type for collect to know what to do, but _ instead of type to infer
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // overwrite example
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // only insert empties example
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating map values example
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
