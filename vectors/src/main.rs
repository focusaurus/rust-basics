fn main() {
    // let names: Vec<String> = Vec::new();
    // names.push(&String::from("Pete"));
    // println!("Vectors {:?}", names);
    let names = vec!["Peter", "Paul", "Mary"];
    println!("Vectors {:?}", names);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("{:?}", v);

    let mut pets = Vec::new();
    pets.push("Winston");
    pets.push("Lily");
    println!("{:?}", pets);
}
