/*
Primitive Types
Integers : u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats   : f32, f64
Boolean  : bool
Characters: char
Tuples
Arrays


Rust is a statically typed language, however the compiler can infer what type we want to use based on the value and how we use it

*/



pub fn run(){
    // Default is "i32"
    let x = 1;
    // Default is "f64"
    let y = 2.5;
    // Add explicit Type
    let z:i64 = 45454545454;

    // Find Max Size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);


    // Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    // Get Boolean from expression
    let is_greater:bool = 10 > 5;

    // Character
    let a1:char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
    
}