pub fn ref_deref() {
    let x = String::from("rust fundamentals");
    // println!("&x: {}", &x);
    let y = &x;
    println!("y: {}", y);
    println!("y: {:p}", y);
    println!("value of x is: {}", *y);
}

pub fn mutable_references() {
    let mut s1 = String::from("hello");
    let r1 = &mut s1;
    println!("r1: {}", r1);
    let r2 = &mut s1;
    println!("r2: {}", r2);
    // Uncommenting the below line will result in a compilation error because the mutable reference r1 is still in scope,
    // and we cannot have multiple mutable references to the same variable in the same scope.
    //println!("r1: {}", r1);
}

pub fn slice_example() {
    let s = String::from("hello world");
    let s1 = &s[0..5];
    let s2 = &s[6..11];
    println!("s1: {}, s2: {}", s1, s2);

    let int_array = [1, 2, 3, 4, 5];
    let slice = &int_array[1..3];
    let first_value = &slice[0];
    println!("slice: {:?}, first_value: {}", slice, first_value);
}

pub fn iteration_behavior() {
    let s = String::from("first word");
    let b = s.as_bytes();

    // Th
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            println!("space found at index: {}", i);
        }
    }

    for (i, ref_item) in b.iter().enumerate() {
        if *ref_item == b' ' {
            println!("space found at index: {}", i);
        }
    }
}
