fn main() {
    // In rust constant must be defined with a type
    const name: &str = "Akila";
    const age: u32 = 30;

    println!("My name is: {}", name);
    println!("I am {} years old", age);
    println!("{} is {} years old", name, age);
}
