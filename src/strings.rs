// Primitive String = Immutable , fixed-length, in memory
// String = Growable, Heap allocated data structure - Use when you need to modify your own data


pub fn run(){
    let hello = "hello"; // immutable fixed length
    let mut hellostring = String::from("Hello");

    // Get Length
    println!("Length: {}", hellostring.len());

    // Push a char
    hellostring.push('!');

    // Push a string
    hellostring.push_str("Huhuhaha");

    // Capacity in bytes
    println!("Capacity: {}", hellostring.capacity());

    // Check if a string is empty
    println!("is empty: {}", hellostring.is_empty());

    // Contains 
    println!("Contains 'World' {}", hellostring.contains("llo"));

    // Replace
    println!("Replace {}", hellostring.replace("llo", "HHH"));


    // Loop through string by whitespace
    for word in hellostring.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", hellostring);

    // Create strring with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

}   