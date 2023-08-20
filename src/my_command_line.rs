
use std::env;

pub fn my_command_line() {
let args: Vec<String> = env::args().collect();

    println!("My path is {}", args[0]);

    let command = args[1].clone();

    let status = "1001";

    if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }   
}

