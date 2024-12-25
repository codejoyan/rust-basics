use std::ops::Bound;

use crossbeam_skiplist::map::Entry;
use crossbeam_skiplist::SkipMap;

mod examples {
    // Import the functions from collection_example.rs
    pub mod collection_example;
    pub mod lifetime_example;
    pub mod ourborous_example;
    pub mod ownership_basics;
}

fn main() {
    // Call the function from collection_example.rs
    // examples::collection_example::collection_example();

    // examples::ownership_basics::iteration_behavior();
    //examples::ownership_basics::slice_example();

    let str1 = String::from("hello");
    {
        let str2 = String::from("world");
        let result = examples::lifetime_example::longest(str1.as_str(), str2.as_str());
        println!("The longest string is {}", result);
        println!("Is str2 {} is still in scope !!", str2);
    }
    examples::ourborous_example::test_ouroborous_func();

    let map = SkipMap::new();

    // Insert some key-value pairs into the map
    map.insert(1, "One");
    map.insert(2, "Two");
    map.insert(3, "Three");
    map.insert(4, "Four");
    map.insert(5, "Five");

    // Iterate over key-value pairs within the range
    for entry in map.range((Bound::Included(&2), Bound::Included(&4))) {
        println!("Key: {}, Value: {}", entry.key(), entry.value());
    }

    // Size of u16
    let SIZEOF_U16: usize = std::mem::size_of::<u16>();
    println!("Size of u16 is {}:", SIZEOF_U16);
}
