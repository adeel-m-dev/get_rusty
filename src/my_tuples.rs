// Tuples group together values of different types
// Max 12 elements
// Fixed length once declared
// Can hold different data types
// Can be used to return multiple values from a function
// Can be used to pass multiple values to a function
// Can be destructured to create individual variables
// Can be nested to create arrays and matrices 

pub fn my_tuples() {
    let user_info:(i32, &str, &str, bool) = (1, "Adeel", "Manzoor", true);
    println!("Record {} is {} {} and is active: {}", user_info.0, user_info.1, user_info.2, user_info.3);
   
}