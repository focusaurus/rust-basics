// struct Album {
//     year: u32,
// }
//
// fn main() {
//     let album = Album { year: 2017 };
//     let a2 = Album { year: 1999 };
//     println!("Album {}", a2.year);
// }
struct Rectangle {
    length: u32,
    width: u32,
}
fn area(rect: Rectangle) -> u32 {
    rect.length * rect.width
}

fn main() {
    let rect = Rectangle {
        length: 32u32,
        width: 42u32,
    };
    println!("The area is {}", area(rect));
}
