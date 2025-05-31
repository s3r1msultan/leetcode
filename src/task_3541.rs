// You are given a string s consisting of lowercase English letters ('a' to 'z').
//
// Your task is to:
//
// Find the vowel (one of 'a', 'e', 'i', 'o', or 'u') with the maximum frequency.
// Find the consonant (all other letters excluding vowels) with the maximum frequency.
//
// Return the sum of the two frequencies.
//
// Note: If multiple vowels or consonants have the same maximum frequency, you may choose any one of them. If there are no vowels or no consonants in the string, consider their frequency as 0.
// The frequency of a letter x is the number of times it occurs in the string.
//
//
//
// Example 1:
//
// Input: s = "successes"
//
// Output: 6
//
// Explanation:
//
// The vowels are: 'u' (frequency 1), 'e' (frequency 2). The maximum frequency is 2.
// The consonants are: 's' (frequency 4), 'c' (frequency 2). The maximum frequency is 4.
// The output is 2 + 4 = 6.
//
// Example 2:
//
// Input: s = "aeiaeia"
//
// Output: 3
//
// Explanation:
//
// The vowels are: 'a' (frequency 3), 'e' ( frequency 2), 'i' (frequency 2). The maximum frequency is 3.
// There are no consonants in s. Hence, maximum consonant frequency = 0.
// The output is 3 + 0 = 3.
//
//
//
// Constraints:
//
// 1 <= s.length <= 100
// s consists of lowercase English letters only.
//

/*pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut map = std::collections::HashMap::<Vec<char>, i32>::new();
    let mut alone_palindromes_map = std::collections::HashMap::<Vec<char>, i32>::new();
    let mut result = 0;

    for s in words {
        let chars = s.chars().collect::<Vec<_>>();
        let converted_chars = chars.clone().into_iter().rev().collect::<Vec<_>>();
        if chars == converted_chars {
            *alone_palindromes_map.entry(chars).or_insert(0) += 1;
            continue;
        }
        if map.contains_key(&converted_chars) {
            let count = map.get_mut(&converted_chars).unwrap();
            if *count == 1 {
                map.remove(&converted_chars);
            } else {
                *count -= 1;
            }
            result += 4;
        } else {
            *map.entry(chars).or_insert(0) += 1;
        }
    }

    let mut is_alone_palindrome = false;

    for &count in map.values() {
        result += 4 * (count / 2);
        if count % 2 == 1 {
            is_alone_palindrome = true;
        }
    }

    if is_alone_palindrome {
        result += 2;
    }

    result
}*/

pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut hash_table = vec![0; 26 * 26];
    let mut result = 0;

    for s in words {
        let a = (s.as_bytes()[0] - b'a') as usize;
        let b = (s.as_bytes()[1] - b'a') as usize;

        let hash = a * 26 + b;
        let inv_hash = b * 26 + a;

        if hash_table[inv_hash] > 0 {
            hash_table[inv_hash] -= 1;
            result += 4;
        } else {
            hash_table[hash] += 1;
        }
    }

    for i in 0..26 {
        let i = i * 26 + i;
        if hash_table[i] > 0 {
            result += 2;
            break;
        }
    }

    result
}