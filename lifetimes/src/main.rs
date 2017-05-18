fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        return a;
    }
    return b;
}

fn main() {
    let string1 = String::from("one1111");
    let string2 = String::from("two22222");
    let result = longest(string1.as_str(), string2.as_str());
    println!("{:?}", result);
}
