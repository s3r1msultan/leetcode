/*You are given a string s that consists of lower case English letters and brackets.

Reverse the strings in each pair of matching parentheses, starting from the innermost one.

Your result should not contain any brackets.



Example 1:

Input: s = "(abcd)"
Output: "dcba"

Example 2:

Input: s = "(u(love)i)"
Output: "iloveu"
Explanation: The substring "love" is reversed first, then the whole string is reversed.

Example 3:

Input: s = "(ed(et(oc))el)"
Output: "leetcode"
Explanation: First, we reverse the substring "oc", then "etco", and finally, the whole string.



Constraints:

1 <= s.length <= 2000
s only contains lower case English characters and parentheses.
It is guaranteed that all parentheses are balanced.

*/



fn reverse_parentheses(s: String) -> String {
	let mut stack = vec![];
	let mut text = "".to_string();
	for char in s.chars() {
		if char == '(' {
			stack.push(text);
			text = "".to_string();
		} else if char == ')' {
			let reversed = text.chars().rev().collect::<String>();
			if let Some(mut prev_text) = stack.pop() {
				prev_text.push_str(&reversed);
				text = prev_text;
			}
		} else {
			text.push(char);
		}
	}
	text
}