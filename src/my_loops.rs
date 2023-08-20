// loops are used to iterate until a condition is met

pub fn my_loops() {
let mut count = 0;

// infinite loop
// loop{
//     count += 1;
//     println!("Number: {}", count);
//     if count == 20 {
//         break;
//     }
// }

// while loop (FizzBuzz)
// while count <= 100 {
//     if count % 15 == 0 {
//         println!("FizzBuzz");
//     } else if count % 3 == 0 {
//         println!("Fizz");
//     } else if count % 5 == 0 {
//         println!("Buzz");
//     } else {
//         println!("{}", count);
//     }
//     // Increment
//      count += 1; 
// }

// for range loop
for x in 0..100 {
    if x % 15 == 0 {
        println!("FizzBuzz");
    } else if x % 3 == 0 {
        println!("Fizz");
    } else if x % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", x);
    }

}

}