fn add(x: i16, y: i16) -> i16 {
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
    // In rust the last statement is implicitly a return statement and 
    //  we need not give semicolon as well
    x + y
}

fn main() {
    // **********
    // Functions
    // **********
    let sum = add(-5, 32);
    println!("The sum is: {}", sum);

    // *************
    // Control Flow
    // *************

    // if-else
    let state_code = 1;
    if state_code == 1 {
        println!("Gujarat");
    } else if state_code == 2 {
        println!("Maharashtra");
    } else {
        println!("Other");
    }

    // Ternary operator in rust
    let status = if state_code < 3 {true} else {false};
    println!("Status: {}", status);

    // Infinite loop
    // loop {
    //     println!("Infinite loop");
    // }

    // Loops can be broken using break, break with value, continue
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Wile loop ended");

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }    

    // for loop with range
    for number in 1..5 {
        println!("loop with range: {}", number);
    }
}