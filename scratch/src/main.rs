fn greet_move_back(name: String) -> String {
    println!("Hello, {}!", name);
    name
}
fn greet_borrow(name: &String) {
    println!("Hello, {}!", &name);
}

fn main () {
    let name = "Pete".to_string();
    let name = greet_move_back(name);
    let name = greet_move_back(name);
    greet_borrow(&name);
}
