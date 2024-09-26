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


// fn longest_common_prefix(strs: Vec<String>) -> String {
// 	if strs.is_empty() {
// 		return "".to_string();
// 	}
//
// 	let mut result = String::new();
// 	let min_len = strs.iter().map(|s| s.len()).min().unwrap_or(0);
//
// 	for i in 0..min_len {
// 		let current_char = strs[0].chars().nth(i).unwrap();
// 		if strs.iter().all(|s| s.chars().nth(i) == Some(current_char)) {
// 			result.push(current_char);
// 		} else {
// 			break;
// 		}
// 	}
//
// 	result
// }

pub fn longest_common_prefix(strs: Vec<String>) -> String {
	struct Trie {
		root: Box<TrieNode>,
	}

	impl Trie {
		pub fn new() -> Self {
			Trie {
				root: Box::new(TrieNode::new())
			}
		}

		pub fn insert(&mut self, string: &str) {
			let mut current_node = &mut self.root;
			for char in string.chars() {
				let i = (char as u8 - 'a' as u8) as usize;
				if current_node.children[i].is_none() {
					current_node.children[i] = Some(Box::new(TrieNode::new()));
				}
				current_node = current_node.children[i].as_mut().unwrap();
				current_node.count += 1;
			}
		}

		pub fn get_longest_common_prefix(&self, n: i32) -> String {
			let mut current_node = &self.root;
			let mut common_prefix = "".to_string();
			loop {
				let node = current_node.children.iter().enumerate().filter(|(_, x)| x.is_some()).filter(|(i, letter)| letter.as_ref().unwrap().count == n).next();
				if let Some((i, next_node)) = node {
					common_prefix.push(('a' as u8 + i as u8) as char);
					current_node = next_node.as_ref().unwrap();
				} else {
					return common_prefix;
				}
			}
		}
	}

	struct TrieNode {
		children: [Option<Box<TrieNode>>; 26],
		count: i32,
	}

	impl TrieNode {
		pub fn new() -> Self {
			TrieNode {
				children: Default::default(),
				count: 0,
			}
		}
	}

	let mut trie = Trie::new();
	let n = strs.len() as i32;
	for string in strs {
		trie.insert(string.as_str());
	}

	trie.get_longest_common_prefix(n)
}

#[cfg(test)]
#[test]

fn test_longest_common_prefix() {
	let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
	let result = "fl".to_string();
	assert_eq!(longest_common_prefix(strs), result);

	let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
	let result = "".to_string();
	assert_eq!(longest_common_prefix(strs), result);
}