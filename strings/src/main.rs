fn main() {
    let mut words = String::from("words");
    words.push_str(" foo");
    println!("strings! {:?}", words);

    let first = "First".to_string();
    let last = "Last".to_string();
    let name = first + " " + &last;
    println!("{:?}", name);
}
