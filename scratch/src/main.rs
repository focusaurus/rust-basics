use std::{fmt,path};

#[derive(Debug)]
struct LearnAsRef<P: AsRef<path::Path>> {
    path: P,
}


impl<'a, L> fmt::Display for &'a LearnAsRef<AsRef<path::Path>> where L: ?Sized {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "LearnAsRef {}", self.path)
    }
}


fn main() {
    let lar = LearnAsRef { path: path::Path::new("foo") };
    println!("{:?}", lar);
}
