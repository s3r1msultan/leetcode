/*You are given a string word and a non-negative integer k.

Return the total number of

of word that contain every vowel ('a', 'e', 'i', 'o', and 'u') at least once and exactly k consonants.



Example 1:

Input: word = "aeioqq", k = 1

Output: 0

Explanation:

There is no substring with every vowel.

Example 2:

Input: word = "aeiou", k = 0

Output: 1

Explanation:

The only substring with every vowel and zero consonants is word[0..4], which is "aeiou".

Example 3:

Input: word = "ieaouqqieaouqq", k = 1

Output: 3

Explanation:

The substrings with every vowel and one consonant are:

word[0..5], which is "ieaouq".
word[6..11], which is "qieaou".
word[7..12], which is "ieaouq".



Constraints:

5 <= word.length <= 2 * 105
word consists only of lowercase English letters.
0 <= k <= word.length - 5
*/

pub fn count_of_substrings(word: String, k: i32) -> i64 {
    fn is_vowel(ch: char) -> bool {
        match ch {
            'a' | 'e' | 'u' | 'o' | 'i' => true,
            _ => false
        }
    }
    let n = word.len();
    let chars = word.chars().collect::<Vec<_>>();
    let mut vowels_map = std::collections::HashMap::new();
    let mut consonant_count = 0;
    let mut substring_count = 0;

    let mut next_consonants = vec![0; n];
    let mut next_consonant_index = n;
    for i in (0..n).rev() {
        next_consonants[i] = next_consonant_index;
        if !is_vowel(chars[i]) {
            next_consonant_index = i;
        }
    }

    let mut left = 0;

    for right in 0..n {
        if !is_vowel(chars[right]) {
            consonant_count += 1;
        } else {
            *vowels_map.entry(chars[right]).or_insert(0) += 1;
        }

        while consonant_count > k {

            if !is_vowel(chars[left]) {
                consonant_count -= 1;
            } else {
                if Some(&1) == vowels_map.get(&chars[left]) {
                    vowels_map.remove(&chars[left]);
                }   else {
                    *vowels_map.entry(chars[left]).or_insert(0) -= 1;
                }
            }

            left += 1;
        }

        while vowels_map.keys().len() == 5 && consonant_count == k {

            substring_count += next_consonants[right] - right;

            if !is_vowel(chars[left]) {
                consonant_count -= 1;
            } else {
                if Some(&1) == vowels_map.get(&chars[left]) {
                    vowels_map.remove(&chars[left]);
                }   else {
                    *vowels_map.entry(chars[left]).or_insert(0) -= 1;
                }
            }

            left += 1;
        }
    }

    substring_count as i64
}