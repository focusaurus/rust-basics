fn largest(numbers: &[i32]) -> i32 {
    let mut result = numbers[0];
    for &number in numbers.iter() {
        if number > result {
            result = number
        }
    }
    result
}

fn main() {
    println!("largest {}", largest(&vec![7, 19, 23, 5]));
    println!("largest {}", largest(&vec![]));
}
