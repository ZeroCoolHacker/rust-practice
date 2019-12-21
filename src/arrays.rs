
//  Fixed list, Same type of elements
use std::mem;

pub fn run(){
    let mut numbers:[i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Reassign value
    numbers[2] = 45;

    // Get single Value 
    println!("Single Value: {}", numbers[0]);

    // Get Array Length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}