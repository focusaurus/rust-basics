use std::iter::FromIterator;

fn normalize(word: &str) -> String {
    let mut lower_letters: Vec<char> = word.to_lowercase().chars().collect();
    lower_letters.sort();
    String::from_iter(lower_letters)
}

pub fn anagrams_for<'a>(base_word: &str, words: &'a [&str]) -> Vec<&'a str> {
    let normal_base = normalize(base_word);
    words
        .iter()
        // Main anagram filter by case normalize and sort letters
        .filter(|&&word| normalize(word) == normal_base)
        // Special case for upper/lower same word isn't an anagram
        .filter(|&&word| word.to_lowercase() != base_word.to_lowercase())
        // ref/deref hoops
        .map(|word| *word)
        .collect::<Vec<&str>>()
}
