
pub fn run(){
    let age:u8 = 23;
    let check_id:bool = false;
    let knows_person_of_age = true;



    // If else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry You have to leave!");
    } else {
        println!("Bartender: I'll need to check your ID");
    }

    // Shorthand IF
    let is_of_age = if age >= 21 { true } else { false };
    println!("is_of_age: {}", is_of_age);
}