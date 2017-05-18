#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: PartialOrd + Copy>(numbers: &[T]) -> T {
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
    println!("largest {}", largest(&vec![67, 3, 99, 101]));
    println!("largest {}", largest(&vec!['a', 'z', '&', '~']));
    println!("largest {}", largest(&vec!['x', 'c']));
    // let p = Point { x: 42, y: 43 };
    let p1 = Point { x: 7, y: 42.5 };
    let p2 = Point { x: "X", y: 'y' };
    let p3 = p1.mixup(p2);
    println!("Point {:?}", p3);
}
