fn calulate_string_length_by_borrowing(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn borrowing_demo() {
    let s1 = String::from("Hello");
    let (s1, len) = calulate_string_length_by_borrowing(s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calulate_string_length_by_referencing(s: &String) -> usize {
    s.len()
}

fn referencing_demo() {
    let s1 = String::from("Hello");
    let len = calulate_string_length_by_referencing(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn change_string_by_mutable_referencing(s: &mut String) {
    s.push_str(", world!");
}

fn mutable_referencing_demo() {
    let mut s = String::from("Hello");
    change_string_by_mutable_referencing(&mut s);

    // Its not possible to have more than one mutable reference to a particular piece of data in a particular scope.
    // This is to avoid the race condition. 
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // But it is possible to have one mutable reference and any number of immutable references in a particular scope.
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
}

fn dangle() -> &String {
    let s = String::from("Hello");
    // The variable s is deallocated here. 
    &s
}

fn dangling_reference_demo() {
    // This will not compile because the reference is pointing to a value that has been deallocated.
    // let reference_to_nothing = dangle();
}

fn main() {
    /*
        References and Borrowing
    */
    borrowing_demo();
    referencing_demo();
    mutable_referencing_demo();
    dangling_reference_demo();
}