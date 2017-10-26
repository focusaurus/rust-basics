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
fn main() {

    for number in 1..101 {
        let message = match (number % 3, number % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => format!("{}", number),
        };
        println!("{}", message);
    }
}
