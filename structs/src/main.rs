// struct Album {
//     year: u32,
// }
//
// fn main() {
//     let album = Album { year: 2017 };
//     let a2 = Album { year: 1999 };
//     println!("Album {}", a2.year);
// }
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

fn main() {
    let rect1 = Rectangle {
        length: 32,
        width: 42,
    };
    let rect2 = Rectangle {
        length: 6,
        width: 7,
    };
    println!("Area: {}", rect1.area());
    println!("Square: {:?}", Rectangle::square(16));
    println!("Can hold? {}", rect1.can_hold(&rect2));
}
