// Group together values of different types
// Max 12 Elements

pub fn run(){
    let person:(&str, &str, i8) = ("Shazy", "Mass", 37);

    println!("{} is from {} and is {} years old", person.0, person.1, person.2);
}