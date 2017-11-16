#[derive(Debug)]
struct Cart<'a> {
    apples: u8,
    raisins: usize,
    description: String,
    location: &'a [u8; 4],
}

fn greet_move_back(name: String) -> String {
    println!("Hello, {}!", name);
    name
}

fn greet_borrow(name: &String) {
    println!("Hello, {}!", &name);
}

fn greet_alloc_and_move() -> String {
    "some data from greet_alloc_and_move".to_string()
}

fn get_cart(bytes: &[u8; 4]) -> Cart {
    Cart {
        apples: 7,
        raisins: 1000,
        description: greet_alloc_and_move(),
        location: bytes,
    }
}

fn main() {
    let name = "Pete".to_string();
    let name = greet_move_back(name);
    let name = greet_move_back(name);
    let data = greet_alloc_and_move();
    println!("from greet_alloc_and_move: {}", data);
    let cart = get_cart(&[0, 1, 2, 3]);
    println!("cart {:?}", cart);
    greet_borrow(&name);
}
