pub fn run(){
    let mut count = 0;

    // 'loop' is Infinite Loop, we need to stop it
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While Loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }

        // increment
        count += 1;
    }


    // For Range
    for x in 0..100 {
        if x%2 == 0 {
            println!("{} is Even", x);
        } else {
            println!("{} is Odd", x);
        }
    }
}