struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// fn largest<T>(numbers: &[T]) -> T {
//     let mut result = numbers[0];
//     for &number in numbers.iter() {
//         if number > result {
//             result = number
//         }
//     }
//     result
// }

fn main() {
    // println!("largest {}", largest(&vec![7, 19, 23, 5]));
    // println!("largest {}", largest(&vec![67, 3, 99, 101]));
    // println!("largest {}", largest(&vec!['a', 'z', '&', '~']));
    // println!("largest {}", largest(&vec!['x', 'c']));
    let p = Point { x: 42, y: 43 };
    println!("Point {}", p.x());
}
