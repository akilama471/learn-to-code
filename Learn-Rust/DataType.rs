fn main() {
    // Reusable function to Print Data Type of variable
    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
    }

    // In Rust, the type of a variable is decided by the value you give it. Rust looks at the value and automatically chooses the right type
    let my_num = 5; // integer
    let my_double = 5.99; // float
    let my_letter = 'D'; // character
    let my_bool = true; // boolean
    let my_text = "Hello"; // string

    //However, it is possible to explicitly tell Rust what type a value should be:
    let i_my_num: i32 = 5; // integer
    let i_my_double: f64 = 5.99; // float
    let i_my_letter: char = 'D'; // character
    let i_my_bool: bool = true; // boolean
    let i_my_text: &str = "Hello"; // string

    print_type_of(&my_num);
    print_type_of(&my_double);
    print_type_of(&my_letter);
    print_type_of(&my_bool);
    print_type_of(&my_text);

    print_type_of(&i_my_num);
    print_type_of(&i_my_double);
    print_type_of(&i_my_letter);
    print_type_of(&i_my_bool);
    print_type_of(&i_my_text);
}
