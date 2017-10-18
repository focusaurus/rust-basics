extern crate external_traits_lib;
use external_traits_lib::x::Point as Point;
use std::fmt;

struct MyPoint (Point);

impl fmt::Binary for MyPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "MP: {:b} {:b}", (self.0).0, (self.0).1)
    }
}
// impl Binary for Point {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//         f.write("Binary Point")
//     }
// }
fn main() {
    // let p = Point{0:7, 1:8};
    // let p = Point(7, 8);
    println!("{:b}", MyPoint(Point(7, 8)));
}
