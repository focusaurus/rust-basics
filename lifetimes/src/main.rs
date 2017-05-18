fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    b
}

fn main() {
    let string1 = String::from("one");
    let string2 = "two222";
    let result = longest(string1.as_str(), string2);
    println!("{:?}", result);
}
