
// Used to create custom data types
// using UPPERCASE is the convention for structs


// Traditional Struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Return Full Name
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last:&str) {
        self.last_name = last.to_string();
    }

    // name tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}



struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct TColor(u8, u8, u8);

pub fn run() {
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Traditional Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TColor(255, 0, 0);
    println!("Tuple Color: {} {} {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("John", "Changar");
    println!("Person {}", p.get_full_name());

    p.set_last_name("Wick");
    println!("Person {}", p.get_full_name());

    println!("Person Tuple {:?}", p.to_tuple());

}

