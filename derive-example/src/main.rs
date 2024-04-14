// use std::path::Display;

use derive_macro::MyDeriveExample;

#[derive(MyDeriveExample)]
struct Student {
    name: String,
    age: u32,
}

// impl Display for Student {
    
// }
fn main() {
    let person = Student {
        name: "Alice".to_string(),
        age: 30,
    };
    person.say_hello()
} 