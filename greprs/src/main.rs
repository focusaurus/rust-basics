use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // for arg in env::args() {
    //     println!("{:?}", arg);
    // }
}
