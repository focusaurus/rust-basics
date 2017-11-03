// fn main() {
//     for number in 1..101 {
//         let mut message = String::new();
//         if number % 3 == 0 {
//             message.push_str("Fizz");
//         }
//         if number % 5 == 0 {
//             message.push_str("Buzz");
//         }
//         if message.len() == 0 {
//             message.push_str(format!("{}", number).as_str());
//         }
//         println!("{}", message);
//     }
// }
// fn main() {
//     for number in 1..101 {
//         let message = match (number % 3, number % 5) {
//             (0, 0) => "FizzBuzz".to_string(),
//             (0, _) => "Fizz".to_string(),
//             (_, 0) => "Buzz".to_string(),
//             _ => format!("{}", number),
//         };
//         println!("{}", message);
//     }
// }
//
// use std::ops::{Generator, GeneratorState};
//
// fn fizzbuzz_generator(upto: u64) -> String {
//     || {
//     for number in 1..upto + 1 {
// match (number % 3, number % 5) {
//             (0, 0) => yield "FizzBuzz".to_string(),
//             (0, _) => yield "Fizz".to_string(),
//             (_, 0) => yield "Buzz".to_string(),
//             _ => yield format!("{}", number),
//         };
//     }
// }
// }
// fn main() {
//     match fizzbuzz_generator(100).resume() {
//         GeneratorState::Yielded(message) => println!("{}", message),
//         GeneratorState::Complete(message) => {println!("{}", message);break;},
//     }
// }

fn main () {

}
