/*You are given two strings s and t such that every character occurs at most once in s and t is a permutation of s.

The permutation difference between s and t is defined as the sum of the absolute difference between the index of the occurrence of each character in s and the index of the occurrence of the same character in t.

Return the permutation difference between s and t.



Example 1:

Input: s = "abc", t = "bac"

Output: 2

Explanation:

For s = "abc" and t = "bac", the permutation difference of s and t is equal to the sum of:

The absolute difference between the index of the occurrence of "a" in s and the index of the occurrence of "a" in t.
The absolute difference between the index of the occurrence of "b" in s and the index of the occurrence of "b" in t.
The absolute difference between the index of the occurrence of "c" in s and the index of the occurrence of "c" in t.

That is, the permutation difference between s and t is equal to |0 - 1| + |2 - 2| + |1 - 0| = 2.

Example 2:

Input: s = "abcde", t = "edbac"

Output: 12

Explanation: The permutation difference between s and t is equal to |0 - 3| + |1 - 2| + |2 - 4| + |3 - 1| + |4 - 0| = 12.



Constraints:

1 <= s.length <= 26
Each character occurs at most once in s.
t is a permutation of s.
s consists only of lowercase English letters.

*/


use std::collections::HashMap;

fn find_permutation_difference(s: String, t: String) -> i32 {
	let mut map = HashMap::new();
	let mut diff = 0;
	for (i, char) in s.chars().enumerate() {
		map.insert(char, i);
	}
	for (i, char) in t.chars().enumerate() {
		diff+=i.abs_diff(*map.get(&char).unwrap());
	}
	diff as i32
}

#[cfg(test)]
#[test]
fn test_find_permutation_difference() {
	let s =  "abc".to_string();
	let t = "bac".to_string();
	let result = 2;
	assert_eq!(find_permutation_difference(s,t),result);

	let s =  "abcde".to_string();
	let t = "edbac".to_string();
	let result = 12;
	assert_eq!(find_permutation_difference(s,t),result);

	let s =  "rwohu".to_string();
	let t = "rwuoh".to_string();
	let result = 4;
	assert_eq!(find_permutation_difference(s,t),result);
}