
// Enums are types which have a few definite values
// Enums are used to create types which can be one of a few variants
// Enums are useful whenever you have a value that can be one of a small set of possible values
// Enums are often used in combination with match to handle all possible cases
// Enums can be defined with the enum keyword
// The values of the variants of an enum are of the same type
// Enum values can be any type: String, numeric, etc.
// Enum values can have different types
// Enum values can have data inside them
// Enums can have methods
// Enums can have associated functions

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Rust is moving up"),
        Movement::Down => println!("Rust is moving down"),
        Movement::Left => println!("Rust is moving left"),
        Movement::Right => println!("Rust is moving right")
    }
}

pub fn my_enums() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    
   
}

