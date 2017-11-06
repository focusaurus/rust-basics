fn once<F>(to_call: F) -> bool
    where F: FnOnce() -> i32
{
    to_call() > 10
}

fn mutable<F>(mut to_call: F) -> bool where F: FnMut(i32) -> i32 {
    to_call(2) > 10
}

fn main() {
    let to_call_once = || 3;
    println!("once: {:?}", once(to_call_once));
    // FnOnce means this would be use after move
    // to_call_once();
    let to_call_mut = |value| value * 2;
    println!("mutable: {:?}", mutable(to_call_mut));
}
