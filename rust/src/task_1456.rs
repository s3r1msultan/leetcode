// Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.
//
// Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.
//
//
//
// Example 1:
//
// Input: s = "abciiidef", k = 3
// Output: 3
// Explanation: The substring "iii" contains 3 vowel letters.
//
// Example 2:
//
// Input: s = "aeiou", k = 2
// Output: 2
// Explanation: Any substring of length 2 contains 2 vowels.
//
// Example 3:
//
// Input: s = "leetcode", k = 3
// Output: 2
// Explanation: "lee", "eet" and "ode" contain 2 vowels.
//
//
//
// Constraints:
//
// 1 <= s.length <= 105
// s consists of lowercase English letters.
// 1 <= k <= s.length
//

use std::collections::HashSet;

pub fn max_vowels(s: String, k: i32) -> i32 {
	let k= k as usize;
	let set = HashSet::from(['a', 'e', 'u', 'o', 'i']);
	let mut max = 0;
	let s: Vec<char> = s.chars().collect();
	for i in 0..k {
		if set.contains(&s[i]) {
			max+=1;
		}
	}
	let mut curr = max;
	let mut left = k;
	while left <= s.len() - 1 {
		if set.contains(&s[left - k]) {
			curr -= 1;
		}
		if set.contains(&s[left]) {
			curr += 1;
		}
		max = max.max(curr);
		left+=1;
	}
	max
}

#[cfg(test)]
#[test]

fn test_max_vowels() {
	let s = "abciiidef".to_string();
	let k = 3;
	let result = 3;
	assert_eq!(max_vowels(s, k), result);

	let s = "aeiou".to_string();
	let k = 2;
	let result = 2;
	assert_eq!(max_vowels(s, k), result);

	let s = "leetcode".to_string();
	let k = 3;
	let result = 2;
	assert_eq!(max_vowels(s, k), result);
}