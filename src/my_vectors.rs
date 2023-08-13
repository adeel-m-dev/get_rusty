
// Vectors - Resizable arrays

use std::mem;

pub fn my_vectors() {
   let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);
    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Re-assign value
    numbers[3] = 20;
    println!("{:?}", numbers);

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    // pop off last value
    numbers.pop();

    // 

    // Loop through vector values
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