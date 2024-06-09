/*Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".



Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.



Constraints:

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] consists of only lowercase English letters.

*/

use std::ops::Index;

fn longest_common_prefix(strs: Vec<String>) -> String {
	if strs.is_empty() {
		return "".to_string();
	}

	let mut result = String::new();
	let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);

	for i in 0..min_len {
		let current_char = strs[0].chars().nth(i).unwrap();
		if strs.iter().all(|s| s.chars().nth(i) == Some(current_char)) {
			result.push(current_char);
		} else {
			break;
		}
	}

	result
}

#[cfg(test)]

#[test]

fn test_longest_common_prefix() {
	let strs = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
	let result = "fl".to_string();
	assert_eq!(longest_common_prefix(strs), result);

	let strs = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
	let result = "".to_string();
	assert_eq!(longest_common_prefix(strs), result);
}