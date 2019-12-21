pub fn run() {
    greetings("Hello", "hassan");

    // Bind Function values to variables
    let get_sum = add(10, 10);
    println!("Sum: {}", get_sum);

    // Closure - We can use outside variables also in closure like n3
    let n3:i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1+n2+n3;
    println!("Closure Sum: {}", add_nums(3, 3));
}


// Function with parameters
fn greetings(greet: &str, name: &str) {
    println!("{} {}, Nice to meet you!", greet, name);
}

// Function with return value
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}