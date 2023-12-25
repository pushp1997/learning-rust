fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn main() {
    /*
        Ownership Rules in Rust:

        1. Each value in Rust has a variable that’s called its owner.
        2. There can only be one owner at a time.
        3. When the owner goes out of scope, the value will be dropped.
    */
    {
        // String type
        let mut s = String::from("Hello");
        s.push_str(", world!");
        println!("{}", s);
    }
    // println!("{}", s); // This will throw an error because 's' is out of scope.

    // Stack & Heap
    let this_string_is_stored_in_stack = "Hello";
    let this_string_is_stored_in_heap = String::from("Hello");
    println!("{}", this_string_is_stored_in_stack);
    println!("{}", this_string_is_stored_in_heap);

    let s1 = this_string_is_stored_in_stack;
    println!("{} -> {}", this_string_is_stored_in_stack, s1); // This will work
    // This is called a 'move', where the value of 'this_string_is_stored_in_heap' is copied to 's2' and 'this_string_is_stored_in_heap' is invalidated.
    let s2 = this_string_is_stored_in_heap; 
    // println!("{} -> {}", this_string_is_stored_in_heap, s2); // This will throw an error

    // Copying
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, world!", s1); // Now, this will not throw an error
    println!("{}, world!", s2);

    // Ownership & Functions
    let str = String::from("This is a string");
    takes_ownership(str);
    // println!("{}", str); // This will throw an error because 's' is out of scope.

    let i = 55;
    makes_copy(i);
    println!("{}", i); // But this doesn't throw error.

    let str = String::from("This is a string");
    let s3 = takes_and_gives_back(str);
    println!("{}", s3);
}