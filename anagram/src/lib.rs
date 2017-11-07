use std::iter::FromIterator;

fn normalize(word: String) -> String {
    let mut lower_letters: Vec<char> = word.to_lowercase().chars().collect();
    lower_letters.sort();
    String::from_iter(lower_letters)
}
// fn normalize<'a>(word: &str) -> &str {
//     let mut lower_letters: &'a Vec<char> = word.to_lowercase().chars().collect();
//     lower_letters.sort();
//     String::from_iter(lower_letters).as_str()
// }
// fn normalize<'a>(word: &'a str) -> &'a str {
//     let mut lower_letters: Vec<char> = word.to_lowercase().chars().collect();
//     lower_letters.sort();
//     String::from_iter(lower_letters).as_str()
// }

pub fn anagrams_for<'a>(base_word: &str, words: &[&str]) -> Vec<&'a str> {
    let normal_base = normalize(String::from(base_word));
    // words
    //     .iter()
    //     .filter(|&&word| normalize(String::from(word)) == normal_base)
    //     .collect::<Vec<&str>>()
    // let hey = words.into_iter().filter(|&word| word.len() > 0).collect::<Vec<&&str>>();
    let hey = words.into_iter().filter(|word| word.len() > 0).collect::<Vec<&&str>>();
    println!("HEY {:?}", hey);
    vec![]
    // hey
}
