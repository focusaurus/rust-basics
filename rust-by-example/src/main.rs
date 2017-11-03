use std::fmt;

#[derive(Debug)]
enum Animal {
    Dog { age: u8 },
    Cat { age: u8 },
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match self {
                   &Animal::Dog { age } => format!("Dog<{}>", age),
                   &Animal::Cat { age } => format!("Cat<{}>", age),
               })
    }
}
// impl From<Animal> for u8 {
//     fn from<Animal>(animal: Animal) -> Self {
//         animal.age
//     }
// }

fn main() {
    let duke = Animal::Dog { age: 1 };
    println!("{}", duke);
}
