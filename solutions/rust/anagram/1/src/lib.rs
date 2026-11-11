use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted_chars(&word_lower);
    let mut hashset = HashSet::new();
    for &i in possible_anagrams{
        let condidate = i.to_lowercase();
        if condidate == word_lower || i.len() != word_lower.len(){
            continue;
        }
        let candidate_char = get_sorted_chars(&condidate);
        if word_sorted == candidate_char{
            hashset.insert(i);
        }
    }
    hashset
}

pub fn get_sorted_chars(word: &str)-> Vec<char>{
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars
}

