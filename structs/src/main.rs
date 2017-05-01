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
}

fn main() {
    let rect = Rectangle {
        length: 32,
        width: 42,
    };
    println!("The area is {} {:?}", rect.area(), rect);
}
