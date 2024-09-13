/*You are given an array of words where each word consists of lowercase English letters.

wordA is a predecessor of wordB if and only if we can insert exactly one letter anywhere in wordA without changing the order of the other characters to make it equal to wordB.

For example, "abc" is a predecessor of "abac", while "cba" is not a predecessor of "bcad".

A word chain is a sequence of words [word1, word2, ..., wordk] with k >= 1, where word1 is a predecessor of word2, word2 is a predecessor of word3, and so on. A single word is trivially a word chain with k == 1.

Return the length of the longest possible word chain with words chosen from the given list of words.



Example 1:

Input: words = ["a","b","ba","bca","bda","bdca"]
Output: 4
Explanation: One of the longest word chains is ["a","ba","bda","bdca"].

Example 2:

Input: words = ["xbc","pcxbcf","xb","cxbc","pcxbc"]
Output: 5
Explanation: All the words can be put in a word chain ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].

Example 3:

Input: words = ["abcd","dbqca"]
Output: 1
Explanation: The trivial word chain ["abcd"] is one of the longest word chains.
["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.



Constraints:

1 <= words.length <= 1000
1 <= words[i].length <= 16
words[i] only consists of lowercase English letters.

*/

fn longest_str_chain(words: Vec<String>) -> i32 {
	fn is_predecessor(word1: &str, word2: &str) -> bool {
		if word1.len() + 1 != word2.len() {
			return false;
		}
		let mut i = 0;
		let mut j = 0;
		let mut diff = 0;
		while i < word1.len() {
			if word1.chars().nth(i).unwrap() != word2.chars().nth(j).unwrap() {
				if diff == 1 {
					return false;
				}
				diff += 1;
				j += 1;
			} else {
				i += 1;
				j += 1;
			}
		}
		true
	}
	let mut words = words;
	words.sort_by_key(|word| word.len());
	let n = words.len();
	let mut dp = vec![1; n];
	let mut result = 1;

	for i in 1..n {
		for j in 0..i {
			if is_predecessor(&words[j], &words[i]) {
				dp[i] = dp[i].max(dp[j] + 1);
			}
		}
		result = result.max(dp[i]);
	}

	result
}