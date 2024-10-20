// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results

 

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // Step 1: Check if the string is empty
        if s.is_empty() {
            return Person::default();
        }

        // Step 2: Split the string into parts
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Person::default();
        }

        // Step 3: Extract name and age from parts
        let name = parts[0].trim();
        let age_str = parts[1].trim();

        // Step 4: Handle missing or empty name
        if name.is_empty() {
            return Person::default();
        }

        // Step 5: Try to parse the age
        if let Ok(age) = age_str.parse::<usize>() {
            Person {
                name: name.to_string(),
                age,
            }
        } else {
            Person::default()
        }
    }
}
