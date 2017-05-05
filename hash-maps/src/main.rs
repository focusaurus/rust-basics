use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<String, i32> = HashMap::new();
    hm.insert("Foo".to_string(), 42);
    println!("Hash maps {:?}", hm);

    let teams = vec!["Red".to_string(), "Blue".to_string(), "Yellow".to_string()];
    let score_start = vec![17, 4, 25];
    let scores: HashMap<_, _> = teams.iter().zip(score_start.iter()).collect();
    println!("{:?}", scores);

    let score = scores.get(&"Blue".to_string());

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
    // println!("{:?}", score.or(Some(7)));

    let text = "Hello world wonderful world";

    let mut word_counts: HashMap<_, _> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_counts);
}
