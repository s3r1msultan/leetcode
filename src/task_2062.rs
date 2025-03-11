/*A substring is a contiguous (non-empty) sequence of characters within a string.

A vowel substring is a substring that only consists of vowels ('a', 'e', 'i', 'o', and 'u') and has all five vowels present in it.

Given a string word, return the number of vowel substrings in word.



Example 1:

Input: word = "aeiouu"
Output: 2
Explanation: The vowel substrings of word are as follows (underlined):
- "aeiouu"
- "aeiouu"

Example 2:

Input: word = "unicornarihan"
Output: 0
Explanation: Not all 5 vowels are present, so there are no vowel substrings.

Example 3:

Input: word = "cuaieuouac"
Output: 7
Explanation: The vowel substrings of word are as follows (underlined):
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"
- "cuaieuouac"



Constraints:

1 <= word.length <= 100
word consists of lowercase English letters only.

*/

pub fn count_vowel_substrings(word: String) -> i32 {
    let n = word.len();
    let chars = word.chars().collect::<Vec<char>>();
    fn is_vowel(ch: char) -> bool {
        ch == 'a' || ch == 'e' || ch == 'u' || ch == 'o' || ch == 'i'
    }
    let mut substring_count = 0;
    let mut vowels_map = std::collections::HashMap::new();
    let mut consonants_map = vec![0; n];
    let mut consonant_index = n;
    for i in (0..n).rev() {
        consonants_map[i] = consonant_index;
        if !is_vowel(chars[i]) {
            consonant_index = i;
        }
    }

    let mut left = 0;

    for right in 0..n {
        if !is_vowel(chars[right]) {
            vowels_map.clear();
            left = right + 1;
            continue;
        }

        *vowels_map.entry(chars[right]).or_insert(0) += 1;

        while vowels_map.keys().len() == 5 {
            substring_count += consonants_map[right] - right;
            if Some(&1) == vowels_map.get(&chars[left]) {
                vowels_map.remove(&chars[left]);
            } else {
                *vowels_map.entry(chars[left]).or_insert(0) -= 1;
            }
            left += 1;
        }
    }

    substring_count as i32
}