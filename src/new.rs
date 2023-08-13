pub fn run() {
    println!("This log is from new file i created");

    // Basic Formating 
    println!("{} is from {}", "Adeel", "Pakistan");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Adeel", "Pakistan", "Code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Adeel", activity = "Football");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug Traits
    println!("{:?}", (12, true, "Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
