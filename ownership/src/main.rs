fn first_word(input: &str) -> &str {
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }
    &input[..]
}

fn main() {
    let mut name = String::from("Pete");
    name.push_str("r Lyons");
    let name2 = name;
    println!("{}", name2);
    let word = first_word(&name2);
    println!("{}", word);
}
