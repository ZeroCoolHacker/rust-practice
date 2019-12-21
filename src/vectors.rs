//  Resizeable Arrays, Same type of elements
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Reassign value
    numbers[2] = 45;

    // Add onto vector
    numbers.push(22);
    numbers.push(452);
    numbers.pop();

    // Get single Value 
    println!("Single Value: {}", numbers[0]);

    // Get Vector Length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack allocated
    println!("This Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    println!("{:?}", numbers);


    // Loop through vector values
    for x in numbers.iter() {
        println!("Number : {}", x);
    }

    // Loop & Mutate values
    for x in numbers.iter_mut() {
        *x *= 2; // deferefence x, then multiply it by 2
    }

    println!("Numbers Vec : {:?}", numbers);
}