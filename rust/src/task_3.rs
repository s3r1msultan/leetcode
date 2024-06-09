/*Given a string s, find the length of the longest
substring
without repeating characters.



Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.



Constraints:

    0 <= s.length <= 5 * 104
    s consists of English letters, digits, symbols and spaces.

*/
use std::collections::{VecDeque, HashSet};
pub fn length_of_longest_substring(s: String) -> i32 {
	let mut queue:VecDeque<char> = VecDeque::new();
	let mut set: HashSet<char> = HashSet::new();
	let mut res = 0;
	for char in s.chars() {

		while set.contains(&char)  {
			set.remove(&queue.pop_front().unwrap());
		}



		queue.push_back(char);
		set.insert(char);
		if queue.len() > res {
			res = queue.len();
		}

	}

	return res as i32;
}
