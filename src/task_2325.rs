// You are given the strings key and message, which represent a cipher key and a secret message, respectively. The steps to decode message are as follows:
//
// Use the first appearance of all 26 lowercase English letters in key as the order of the substitution table.
// Align the substitution table with the regular English alphabet.
// Each letter in message is then substituted using the table.
// Spaces ' ' are transformed to themselves.
//
// For example, given key = "happy boy" (actual key would have at least one instance of each letter in the alphabet), we have the partial substitution table of ('h' -> 'a', 'a' -> 'b', 'p' -> 'c', 'y' -> 'd', 'b' -> 'e', 'o' -> 'f').
//
// Return the decoded message.
//
//
//
// Example 1:
//
// Input: key = "the quick brown fox jumps over the lazy dog", message = "vkbs bs t suepuv"
// Output: "this is a secret"
// Explanation: The diagram above shows the substitution table.
// It is obtained by taking the first appearance of each letter in "the quick brown fox jumps over the lazy dog".
//
// Example 2:
//
// Input: key = "eljuxhpwnyrdgtqkviszcfmabo", message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb"
// Output: "the five boxing wizards jump quickly"
// Explanation: The diagram above shows the substitution table.
// It is obtained by taking the first appearance of each letter in "eljuxhpwnyrdgtqkviszcfmabo".
//
//
//
// Constraints:
//
// 26 <= key.length <= 2000
// key consists of lowercase English letters and ' '.
// key contains every letter in the English alphabet ('a' to 'z') at least once.
// 1 <= message.length <= 2000
// message consists of lowercase English letters and ' '.
//


use std::collections::HashMap;

pub fn decode_message(key: String, message: String) -> String {
    let key = key.replace(" ", "").chars().collect::<Vec::<char>>();
    let mut table: HashMap<char, char> = HashMap::new();
    let alphabet = ('a'..='z').collect::<Vec<char>>();
    let mut alphabet = alphabet.iter();
    for letter in key {
        if !table.contains_key(&letter) {
            table.insert(letter, *alphabet.next().unwrap());
        }
    }
    let mut decipher: String = String::new();
    for letter in message.chars() {
        let decipher_letter = table.get(&letter).unwrap_or(&' ');
        decipher.push(*decipher_letter);
    }
    decipher
}

#[cfg(test)]
mod tests {
    use crate::task_2325::decode_message;
    #[test]
    fn test_decode_message() {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".to_string();
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string();
        assert_eq!(decode_message(key, message), "the five boxing wizards jump quickly".to_string());

        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        assert_eq!(decode_message(key, message), "this is a secret".to_string())
    }
}

