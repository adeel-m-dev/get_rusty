
// Structs used to create custom data types (similar to classes in other languages)

pub fn my_structs() {

    struct Color {
        red: u8,
        green: u8,
        blue: u8
    }

    // Tuple Struct
    struct Color2(u8, u8, u8);

    // Structs with functions
    struct Person {
        first_name: String,
        last_name: String
    }


    impl Person {
        // Construct person
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string()
            }
        }

        // Get full name
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        // Set last name
        fn set_last_name(&mut self, last: &str) {
            self.last_name = last.to_string();
        }

        // Name to tuple
        fn to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }
    }       

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(255, 0, 0);

    c2.0 = 200;

    println!("Color: {} {} {}", c2.0, c2.1, c2.2);

    let mut p = Person::new("Adeel", "Manzoor");

    println!("Person {}", p.full_name());


    p.set_last_name("Jugnu");

    println!("Person {}", p.full_name());


    println!("Person Tuple {:?}", p.to_tuple());




}

