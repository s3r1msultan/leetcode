/*Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.

Note that the strings are case-insensitive, both lowercased and uppercased of the same letter are treated as if they are at the same row.

In the American keyboard:

the first row consists of the characters "qwertyuiop",
the second row consists of the characters "asdfghjkl", and
the third row consists of the characters "zxcvbnm".



Example 1:

Input: words = ["Hello","Alaska","Dad","Peace"]

Output: ["Alaska","Dad"]

Explanation:

Both "a" and "A" are in the 2nd row of the American keyboard due to case insensitivity.

Example 2:

Input: words = ["omk"]

Output: []

Example 3:

Input: words = ["adsdf","sfd"]

Output: ["adsdf","sfd"]



Constraints:

1 <= words.length <= 20
1 <= words[i].length <= 100
words[i] consists of English letters (both lowercase and uppercase).

*/
use std::collections::HashSet;

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let first_row = HashSet::from(['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p']);
    let second_row = HashSet::from(['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l']);

    fn get_the_row(ch: char, first_row: &HashSet<char>, second_row: &HashSet<char>) -> i32 {
        let ch = ch.to_ascii_lowercase();
        if first_row.contains(&ch) {
            1
        } else if second_row.contains(&ch) {
            2
        } else {
            3
        }
    }

    let mut result = vec![];

    for word in words {
        let mut chars = word.chars();
        let initial_row = get_the_row(chars.next().unwrap(), &first_row, &second_row);
        let mut is_the_same_row = true;
        for ch in chars {
            if initial_row != get_the_row(ch, &first_row, &second_row) {
                is_the_same_row = false;
                break;
            }
        }
        if is_the_same_row {
            result.push(word.clone());
        }
    }

    result
}
