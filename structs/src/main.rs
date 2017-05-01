// struct Album {
//     year: u32,
// }
//
// fn main() {
//     let album = Album { year: 2017 };
//     let a2 = Album { year: 1999 };
//     println!("Album {}", a2.year);
// }
fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn main() {
    let length1: u32 = 32;
    let width1: u32 = 42;
    println!("The area is {}", area(length1, width1));
}
