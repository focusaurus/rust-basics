#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    Arkansas,
    Colorado,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        Some(num) => Some(num + 1),
        None => None,
    }
}

fn main() {
    println!("Hi. {}", value_in_cents(Coin::Quarter(UsState::Colorado)));
    let hey = match plus_one(None) {
        None => "None".to_string(),
        Some(x) => x.to_string(),
    };
    println!("Hi. {}", hey);
    let house = Some(5);
    if let house = Some(5) {
        println!("foo")
    }
}
