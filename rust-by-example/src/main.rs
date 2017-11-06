fn once<F>(to_call: F) -> bool
    where F: FnOnce() -> i32
{
    to_call() > 10
}

fn main() {
    let input = 7;
    let my_to_call = || 3;
    println!("{:?}", once(my_to_call));
}
