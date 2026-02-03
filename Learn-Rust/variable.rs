fn main() {
    let name = "Akila";
    let age = 30;

    println!("My name is: {}", name);
    println!("I am {} years old", age);
    println!("{} is {} years old", name, age);

    //By default, variables in Rust cannot be changed after they are created:
    //If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable):
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);
}
