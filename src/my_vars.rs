// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language


pub fn my_vars() {
let name = "Adeel";
let mut age = 23;

println!("My name is {} and i am {}", name, age);
let father_name = "Muhammad Jaffar";
age = 51;

println!("My father name is {} and he is {}", father_name, age);

// Define Constant
// when you define a constant you have to define the data type and you can't use mut keyword with it and you have to define it in uppercase
const ID: i32 = 001;
println!("ID: {}", ID);

// Assign Multiple Variables
let (my_name, my_age) = ("Adeel", 23);
println!("{} is {}", my_name, my_age);


}