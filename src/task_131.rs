/*Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.



Example 1:

Input: s = "aab"
Output: [["a","a","b"],["aa","b"]]

Example 2:

Input: s = "a"
Output: [["a"]]



Constraints:

1 <= s.length <= 16
s contains only lowercase English letters.

*/

pub fn partition(s: String) -> Vec<Vec<String>> {
	let mut current = vec![];
	let mut result = vec![];
	backtrack(0, &s, &mut current, &mut result);
	result
}

fn is_palindrome(s: &str) -> bool {
	s.chars().eq(s.chars().rev())
}

fn backtrack(start: usize, s: &String, current: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
	if start == s.len() {
		result.push(current.clone());
		return;
	}

	for end in start+1..=s.len() {
		let substring = &s[start..end];
		if is_palindrome(substring) {
			current.push(substring.to_string());
			backtrack(end, s, current, result);
			current.pop();
		}
	}
}