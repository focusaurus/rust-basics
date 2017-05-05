fn largest_i32(numbers: &[i32]) -> i32 {
    let mut result = numbers[0];
    for &number in numbers.iter() {
        if number > result {
            result = number
        }
    }
    result
}

fn largest_char(chars: &[char]) -> char {
    let mut result = chars[0];
    for &character in chars.iter() {
        if character > result {
            result = character
        }
    }
    result
}

fn main() {
    println!("largest {}", largest_i32(&vec![7, 19, 23, 5]));
    println!("largest {}", largest_i32(&vec![67, 3, 99, 101]));
    println!("largest {}", largest_char(&vec!['a', 'z', '&', '~']));
    println!("largest {}", largest_char(&vec!['x', 'c']));
}
