pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    fn is_vowel(ch: char) -> bool {
        match ch {
            'a' | 'e' | 'o' | 'i' | 'u' => true,
            _ => false,
        }
    }

    let mut set = HashSet::new();
    let mut capitalizaiton_errors_map = HashMap::new();
    let mut vowel_errors_map = HashMap::new();

    for word in wordlist.iter() {
        set.insert(word.clone());
        let lowercase_word = word.to_lowercase();
        if !capitalizaiton_errors_map.contains_key(&lowercase_word) {
            capitalizaiton_errors_map.insert(lowercase_word.clone(), word.clone());
        }
        let mut new_word = String::new();
        for ch in lowercase_word.chars() {
            if is_vowel(ch) {
                new_word.push('a');
            } else {
                new_word.push(ch);
            }
        }
        if !vowel_errors_map.contains_key(&new_word) {
            vowel_errors_map.insert(new_word, word.clone());
        }
    }

    let mut result: Vec<String> = vec![];

    for query in queries {
        if set.contains(&query) {
            result.push(query);
            continue;
        }

        let lowercase_query = query.to_lowercase();

        if let Some(word) = capitalizaiton_errors_map.get(&lowercase_query) {
            result.push(word.clone());
            continue;
        }

        let mut new_word = String::new();

        for ch in lowercase_query.chars() {
            if is_vowel(ch) {
                new_word.push('*');
            } else {
                new_word.push(ch);
            }
        }

        if let Some(word) = vowel_errors_map.get(&new_word) {
            result.push(word.clone());
        } else {
            result.push(String::new());
        }
    }

    result
}
