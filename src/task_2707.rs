/*You are given a 0-indexed string s and a dictionary of words dictionary. You have to break s into one or more non-overlapping substrings such that each substring is present in dictionary. There may be some extra characters in s which are not present in any of the substrings.

Return the minimum number of extra characters left over if you break up s optimally.



Example 1:

Input: s = "leetscode", dictionary = ["leet","code","leetcode"]
Output: 1
Explanation: We can break s in two substrings: "leet" from index 0 to 3 and "code" from index 5 to 8. There is only 1 unused character (at index 4), so we return 1.

Example 2:

Input: s = "sayhelloworld", dictionary = ["hello","world"]
Output: 3
Explanation: We can break s in two substrings: "hello" from index 3 to 7 and "world" from index 8 to 12. The characters at indices 0, 1, 2 are not used in any substring and thus are considered as extra characters. Hence, we return 3.



Constraints:

1 <= s.length <= 50
1 <= dictionary.length <= 50
1 <= dictionary[i].length <= 50
dictionary[i] and s consists of only lowercase English letters
dictionary contains distinct words

*/
use std::usize;

pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
	#[derive(Debug)]
	pub struct Trie {
		root: TrieNode,
	}
	#[derive(Debug, Clone)]
	struct TrieNode {
		children: Vec<Option<Box<TrieNode>>>,
		is_end_of_word: bool,
	}
	impl TrieNode {
		fn new() -> Self {
			TrieNode {
				children: vec![None; 26],
				is_end_of_word: false,
			}
		}
	}
	impl Trie {
		pub fn new() -> Self {
			Trie {
				root: TrieNode::new()
			}
		}

		pub fn insert(&mut self, word: &str) {
			let mut current_node = &mut self.root;
			for c in word.chars() {
				let i = (c as u8 - 'a' as u8) as usize;
				if current_node.children[i].is_none() {
					current_node.children[i] = Option::from(Box::new(TrieNode::new()));
				}
				current_node = current_node.children[i].as_mut().unwrap();
			}
			current_node.is_end_of_word = true;
		}

		pub fn contains(&self, word: &str) -> bool {
			let mut current_node = &self.root;
			for c in word.chars() {
				let i = (c as u8 - 'a' as u8) as usize;
				if current_node.children[i].is_none() {
					return false;
				}
				current_node = current_node.children[i].as_ref().unwrap();
			}
			current_node.is_end_of_word
		}
	}
	use std::collections::HashSet;
	let mut set = HashSet::new();
	for word in dictionary.iter() {
		set.insert(word.as_str());
	}
	let n = s.len();
	let mut dp = vec![0; n + 1];
	for i in 1..=n {
		dp[i] = i;
	}
	for i in 1..=n {
		dp[i] = dp[i - 1] + 1;
		for j in 0..i {
			if set.contains(&s[j..i]) {
				dp[i] = dp[i].min(dp[j]);
			}
		}
	}
	dp[n] as i32
}