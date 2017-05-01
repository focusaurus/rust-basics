fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("Functions {}", y);
    another_function(42);
}

fn another_function(number: i32) {
    println!("nummber: {}", number);
}
