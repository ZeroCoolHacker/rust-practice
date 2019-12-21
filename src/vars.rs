// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a Block Scoped Language



pub fn run() {
    let name = "Shahzaib";
    let mut age  = 21;

    println!("My name is {} and i am {} years old", name, age);
    
    // age var wont change without adding mut
    age = 38;

    println!("My name is {} and i am {} years old", name, age);

    // Define Constants
    //  Constant do need a type to begin with
    const ID: i32 = 001;
    println!("Const {}", ID);


    // Multiple Assignment
    let (my_name, my_age) = ("Ashir", 23);
    println!("my name is {} and i am {}", my_name, my_age);

}