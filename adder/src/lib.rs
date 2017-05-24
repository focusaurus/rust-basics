mod more_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_the_name() {
        // foo
    }

    // #[test]
    // fn oh_crap() {
    //     panic!("poop");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            length: 8,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(17), 19);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);
        assert!(result.contains(name),
                "Greeting did not contain the name {}",
                name);
    }

    #[test]
    #[should_panic(expect = "Too lNOPE")]
    fn guess_should_panic_with_0() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expect = "Too high")]
    fn guess_should_panic_with_101() {
        println!("{:?}", "guess_should_panic_with_101 printed this");
        Guess::new(101);
    }
}

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be in range 1-100. Got {}. Too low.",
                   value);
        }
        if value > 100 {
            panic!("Guess value must be in range 1-100. Got {}. Too high.",
                   value);
        }
        Guess { value: value }
    }
}
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {
            length: side,
            width: side,
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(value: i32) -> i32 {
    value + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // String::from("Hello!")

}
