// conditional - used to check the condition of something and act on the result

pub fn my_conditions() {
 let age: u8 = 18;
 let check_id: bool = true;
 let knows_person_of_age = true;
 
    if age >= 21 && check_id || knows_person_of_age {
    println!("Adeel: Please enter");
    } else if age < 21 && age >= 18  && check_id {
    println!("Adeel: Sorry, you need to show me your ID.");
    } else {
    println!("Adeel: Sorry, you are too young to enter.");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
    
}