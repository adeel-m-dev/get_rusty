
// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn my_arrays() {
   let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Re-assign value
    numbers[3] = 20;
    println!("{:?}", numbers);

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    // Loop through array values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
        *x += 2;
        *x -= 3;
    }
    println!("Numbers Vec: {:?}", numbers);


}