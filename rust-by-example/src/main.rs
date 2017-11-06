fn once<F>(to_call: F) -> bool
    where F: FnOnce() -> i32
{
    to_call() > 10
}

fn mutable<F>(mut to_call: F) -> bool
    where F: FnMut(i32) -> i32
{
    to_call(2) > 10
}

fn by_ref<F>(to_call: F) -> bool
    where F: Fn(&i32) -> i32
{
    to_call(&2) > 10
}

fn by_ref2(input: &i32) -> i32 {
    input * 3
}

fn main() {
    let to_call_once = || 3;
    println!("once: {:?}", once(to_call_once));
    // FnOnce means this would be use after move
    // to_call_once();
    let to_call_mut = |value| value * 2;
    println!("mutable: {:?}", mutable(to_call_mut));

    let to_call_ref = |value: &i32| value * 3;
    println!("by_ref: {:?}", by_ref(by_ref2));
    println!("by_ref: {:?}", by_ref(to_call_ref));
    println!("by_ref2: {:?}", by_ref2(&23));

}
