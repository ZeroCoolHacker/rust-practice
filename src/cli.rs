use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: tuts <arg1> <arg2>");
        std::process::exit(1);
    }
    let command = args[1].clone();
    let name = args[2].clone();

    println!("Command {} {}", command, name);
}