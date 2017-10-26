fn main() {
    for number in 1..101 {
        let mut message = String::new();
        if number % 3 == 0 {
            message.push_str("Fizz");
        }
        if number % 5 == 0 {
            message.push_str("Buzz");
        }
        if message.len() == 0 {
            message.push_str(format!("{}", number).as_str());
        }
        println!("{}", message);
    }
}
