// Primitive Types-- 
// 1. Integers (u8, i8, u16, i16, u32, i32, u64, i64, u128, i128) -- Number of bits they take in memory
// 2. Floats (f32, f64) -- Number of bits they take in memory
// 3. Boolean (bool) -- true or false
// 4. Characters (char) -- single character
// 5. Tuples -- group of values of different types (max 12 elements) -- fixed length once declared can't grow or shrink in size -- can hold different data types -- can be used to return multiple values from a function -- can be used to pass multiple values to a function -- can be destructured to create individual variables -- can be nested to create arrays and matrices     
// 6. Arrays -- group of values of same data types (max 32 elements) -- fixed length once declared can't grow or shrink in size -- can hold same data types -- can be used to return multiple values from a function -- can be used to pass multiple values to a function -- can be destructured to create individual variables -- can be nested to create arrays and matrices 

// rust is a statically typed language which means that it must know the types of all variables at compile time, however the compiler can usually infer what type we want to use based on the value and how we use it


pub fn my_data_types() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 454545454545454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

    

}