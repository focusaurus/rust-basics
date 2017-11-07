use std::iter::FromIterator;
use std::ops::Deref;

fn normalize(word: &str) -> String {
    let mut lower_letters: Vec<char> = word.to_lowercase().chars().collect();
    lower_letters.sort();
    String::from_iter(lower_letters)
}

pub fn anagrams_for<'a>(base_word: &str, words: &'a [&str]) -> Vec<&'a str> {
    let normal_base = normalize(base_word);
    let temp = words
        .iter()
        .filter(|&&word| normalize(word) == normal_base)
        .collect::<Vec<&&str>>();
    let anagrams = temp.iter().map(|item| **item).collect::<Vec<&str>>();
    println!("anagrams: {:?}", anagrams);
    anagrams
}
