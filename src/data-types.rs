
fn return_some_value() -> u8 {
    return 10;
}

fn main() {
    /*
     * Variables & Mutability
     */

    // By default the variables are immutable
    let var_a: u8 = 12;
    // This throws error:
    // var_a = return_some_value();
    println!("Variable a = {}", var_a);
    // This is called shadowing
    let var_a: u8 = 15;
    println!("Variable a = {}", var_a);
    let var_a = "Hello";
    println!("Variable a = {}", var_a);

    // To make a variable immutable add "mut" keyowrd
    let mut var_b: u8 = 12;
    println!("Variable b = {}", var_b);
    var_b = return_some_value();
    println!("Variable b = {}", var_b);

    // Constants are immutable like variables and  must be type annotated
    const PERMANENT_VALUE_A: u8 = 16;
    println!("Permanent value = {}", PERMANENT_VALUE_A);
    // Constants have to be statically assigned when declared. This will throw error:
    // const PERMANENT_VALUE_B: u16 = return_some_value();
    // println!("Permanent value = {}", PERMANENT_VALUE_B);


    /*
     * Scaler Data Types
     */

    // Integers
    let a = 98322; // Can also be written as a = 98_322 
    let b = 0xff; // Hexa
    let c = 0o777; // Octa
    let d = 0b111_000_111; // Binary
    println!("{}, {}, {}, {}", a, b, c, d);

    // Floating points
    let e = 43.2; // Default is f64
    let f: f32 = 12.34;
    println!("{}, {}", e, f);

    // Booleans
    let tr = true;
    let fl: bool = false;
    println!("{}, {}", tr, fl);

    // Character
    let ch = 'z';
    let emoji = '😊';
    println!("{}, {}", ch, emoji);

    /*
     * Compound Data Types
     */

    //  Tuples: Fixed size list with related data of different data types
    let person = ("Pushp", 26);
    let (name, age) = person; // Destructuring
    println!("{}, {}", name, age);
    let age = person.1; // dot notation
    println!("{}, {}", name, age);

    // Arrays: Fixed and homogeneous
    let names = ["Pushp", "Aashna", "Akshay"];
    let aashna = names[1];
    println!("{}", aashna );
    let arr = [0; 8];
    println!("{:?}", arr );
}
