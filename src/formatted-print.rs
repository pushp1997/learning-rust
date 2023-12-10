fn main() {
    // This is an example of a line comment.

    /* 
     * Block comments which go to the closing delimiter.
     */

    println!("Hello World!");
    
    print!("I'm a Rustacean!");
    println!("This should come in the same line.");

    let pushp = "Pushp";
    let aashna = "Aashna";
    println!("Hello {pushp}");
    println!("Hello {}", pushp);
    // Positional argument
    println!("{0}: Hi {1}, how are you?\n{1}: I am grand! Yourself?", pushp, aashna);
    //Named argument
    println!("{person1}: Hi {person2}, how are you?\n{person2}: I am grand! Yourself?", person1=pushp, person2=aashna);

    // Padding format
    println!("Pi is roughly {pi:.precision$}", pi=3.141592, precision=3);

    eprintln!("This is an error message.");
}
