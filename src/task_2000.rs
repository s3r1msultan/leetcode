// Given a 0-indexed string word and a character ch, reverse the segment of word that starts at index 0 and ends at the index of the first occurrence of ch (inclusive). If the character ch does not exist in word, do nothing.
//
// For example, if word = "abcdefd" and ch = "d", then you should reverse the segment that starts at 0 and ends at 3 (inclusive). The resulting string will be "dcbaefd".
//
// Return the resulting string.
//
//
//
// Example 1:
//
// Input: word = "abcdefd", ch = "d"
// Output: "dcbaefd"
// Explanation: The first occurrence of "d" is at index 3.
// Reverse the part of word from 0 to 3 (inclusive), the resulting string is "dcbaefd".
//
// Example 2:
//
// Input: word = "xyxzxe", ch = "z"
// Output: "zxyxxe"
// Explanation: The first and only occurrence of "z" is at index 3.
// Reverse the part of word from 0 to 3 (inclusive), the resulting string is "zxyxxe".
//
// Example 3:
//
// Input: word = "abcd", ch = "z"
// Output: "abcd"
// Explanation: "z" does not exist in word.
// You should not do any reverse operation, the resulting string is "abcd".
//
//
//
// Constraints:
//
// 1 <= word.length <= 250
// word consists of lowercase English letters.
// ch is a lowercase English letter.
//

// Straightforward solution
// pub fn reverse_prefix(word: String, ch: char) -> String {
//     let index = word.find(ch).unwrap_or(0);
//     word[..index+1].chars().rev().collect::<String>() + &*word[index + 1..].to_string()
// }


pub fn reverse_prefix(word: String, ch: char) -> String {
    if let Some(index) = word.find(ch) {
        let mut stack: Vec<char> = vec![];
        for c in word.chars().take(index+1) {
            stack.push(c);
        }

        let mut reversed_substring = String::new();
        while !stack.is_empty() {
            reversed_substring.push(stack.pop().unwrap());
        }

        reversed_substring + &word[index+1..]

    } else {
        word
    }
}

#[cfg(test)]

#[test]

fn test_reverse_prefix() {
    let word = "xyxzxe".to_string();
    let ch = 'z';
    assert_eq!(reverse_prefix(word, ch), "zxyxxe".to_string());

    let word = "abcdefd".to_string();
    let ch = 'd';
    assert_eq!(reverse_prefix(word, ch), "dcbaefd".to_string());

    let word = "abcd".to_string();
    let ch = 'z';
    assert_eq!(reverse_prefix(word, ch), "abcd".to_string());
}

















