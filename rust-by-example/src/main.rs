use std::fmt;
use std::convert;

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

impl convert::From<u8> for Animal {
    fn from(age: u8) -> Self {
        Animal::Dog { age }
    }
}

impl convert::From<String> for Animal {
    fn from(age: String) -> Self {
        Animal::Dog { age: age.parse::<u8>().unwrap() }
    }
}


fn main() {
    let duke = Animal::Dog { age: 1 };
    let marma: Animal = 34.into();
    let lily: Animal = String::from("33").into();
    println!("{}", duke);
    println!("{}", marma);
    println!("{}", lily);
}
