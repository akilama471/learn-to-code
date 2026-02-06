fn main() {
    // Use if to specify a block of code to be executed if a condition is true.
    if 7 > 5 {
        println!("7 is greater than 5.");
    }

    let x = 7;
    let y = 5;

    if x > y {
        println!("x is greater than y.");
    }

    // If the condition is not true, you can use else to run different code:
    let age = 16;

    if age >= 18 {
        println!("You can vote.");
    } else {
        println!("You are too young to vote.");
    }

    // You can check multiple conditions using else if:
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    //Using if as an Expression
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);

    // Simplified Syntax
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);

    // The value from if and else must be the same type, like two pieces of text or two numbers (in the example above, both are strings).
    let number = 5;
    let result = if number < 10 { "Too small" } else { 100 };
    println!("{}", result);
}
