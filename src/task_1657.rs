/*Two strings are considered close if you can attain one from the other using the following operations:

Operation 1: Swap any two existing characters.
For example, abcde -> aecdb
Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)

You can use the operations on either string as many times as necessary.

Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.



Example 1:

Input: word1 = "abc", word2 = "bca"
Output: true
Explanation: You can attain word2 from word1 in 2 operations.
Apply Operation 1: "abc" -> "acb"
Apply Operation 1: "acb" -> "bca"

Example 2:

Input: word1 = "a", word2 = "aa"
Output: false
Explanation: It is impossible to attain word2 from word1, or vice versa, in any number of operations.

Example 3:

Input: word1 = "cabbba", word2 = "abbccc"
Output: true
Explanation: You can attain word2 from word1 in 3 operations.
Apply Operation 1: "cabbba" -> "caabbb"
Apply Operation 2: "caabbb" -> "baaccc"
Apply Operation 2: "baaccc" -> "abbccc"



Constraints:

1 <= word1.length, word2.length <= 105
word1 and word2 contain only lowercase English letters.

*/


fn close_strings(word1: String, word2: String) -> bool {
	if word1.len() != word2.len() {
		return false;
	}

	use std::collections::{HashMap, HashSet};
	let mut word1: Vec<_> = word1.chars().collect();
	let mut word2: Vec<_> = word2.chars().collect();

	let mut letter_counts_1 = HashMap::new();
	let mut letter_counts_2 = HashMap::new();
	let mut set1 = HashSet::new();
	let mut set2 = HashSet::new();

	word1.iter().for_each(|&char| {
		*letter_counts_1.entry(char).or_insert(0) += 1;
		set1.insert(char);
	});
	word2.iter().for_each(|&char| {
		*letter_counts_2.entry(char).or_insert(0) += 1;
		set2.insert(char);
	});

	if set1 != set2 {
		return false;
	}

	let mut counts1: Vec<_> = letter_counts_1.values().collect();
	let mut counts2: Vec<_> = letter_counts_2.values().collect();

	counts1.sort_unstable();
	counts2.sort_unstable();

	counts1 == counts2
}