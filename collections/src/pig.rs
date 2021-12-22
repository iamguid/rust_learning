use std::string::String;
use std::vec::Vec;

const VOWELS: [char;6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub fn convert(s: &String) -> String {
    let mut result_words: Vec<String> = Vec::new();
    let splitted_words: Vec<&str> = s.split_whitespace().collect();

    for elem in splitted_words {
        let mut result_string: String = String::new();

        if starts_with_vowel(elem) {
            result_string.push_str(elem);
            result_string.push_str("-hay");
        } else {
            let first_char: &str = &elem[0..1];
            let word_part: &str = &elem[1..];
            result_string.push_str(word_part);
            result_string.push_str("-");
            result_string.push_str(first_char);
            result_string.push_str(first_char);
        }

        result_words.push(result_string);
    }

    result_words.join(" ")
}

fn starts_with_vowel(s: &str) -> bool {
    let first_char = s.chars().next().unwrap();
    
    for vowel in VOWELS {
        if first_char == vowel {
            return true
        }
    }

    false
}