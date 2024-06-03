/*You are given two strings s and t consisting of only lowercase English letters.

Return the minimum number of characters that need to be appended to the end of s so that t becomes a subsequence of s.

A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.



Example 1:

Input: s = "coaching", t = "coding"
Output: 4
Explanation: Append the characters "ding" to the end of s so that s = "coachingding".
Now, t is a subsequence of s ("coachingding").
It can be shown that appending any 3 characters to the end of s will never make t a subsequence.

Example 2:

Input: s = "abcde", t = "a"
Output: 0
Explanation: t is already a subsequence of s ("abcde").

Example 3:

Input: s = "z", t = "abcde"
Output: 5
Explanation: Append the characters "abcde" to the end of s so that s = "zabcde".
Now, t is a subsequence of s ("zabcde").
It can be shown that appending any 4 characters to the end of s will never make t a subsequence.



Constraints:

1 <= s.length, t.length <= 105
s and t consist only of lowercase English letters.

*/


fn append_characters(s: String, t: String) -> i32 {
	let mut s = s.as_bytes();
	let mut t = t.as_bytes();
	let mut count = t.len() as i32;
	let mut j = 0;
	let mut i = s.iter().position(|&x| x == t[0]).unwrap_or((count - 1) as usize);
	while i < s.len() && j < t.len() {
		if s[i] == t[j] {
			count -= 1;
			j+=1;
		}
		i+=1;
	}
	count
}


#[cfg(test)]

#[test]

fn test_append_characters() {
	let s = "coaching".to_string();
	let t = "coding".to_string();
	let result = 4;
	assert_eq!(append_characters(s, t), result);

	let s = "abcde".to_string();
	let t = "a".to_string();
	let result = 0;
	assert_eq!(append_characters(s, t), result);

	let s = "z".to_string();
	let t = "abcde".to_string();
	let result = 5;
	assert_eq!(append_characters(s, t), result);

	let s = "lbg".to_string();
	let t = "g".to_string();
	let result = 0;
	assert_eq!(append_characters(s, t), result);
}