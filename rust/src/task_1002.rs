/*Given a string array words, return an array of all characters that show up in all strings within the words (including duplicates). You may return the answer in any order.



Example 1:

Input: words = ["bella","label","roller"]
Output: ["e","l","l"]

Example 2:

Input: words = ["cool","lock","cook"]
Output: ["c","o"]



Constraints:

1 <= words.length <= 100
1 <= words[i].length <= 100
words[i] consists of lowercase English letters.

*/

use std::collections::HashMap;

fn common_chars(words: Vec<String>) -> Vec<String> {
	let mut count = vec![u8::MAX; 26];
	for word in words {
		let mut temp = vec![0;26];
		for ch in word.chars() {
			temp[(ch as usize) - ('a' as usize)] += 1;
		}
		for i in 0..26 {
			count[i] = count[i].min(temp[i])
		}
	}
	let mut result: Vec<String> = vec![];
	for (n, &i) in count.iter().enumerate() {
		for _ in 0..i {
			result.push(((b'a' + n as u8) as char).to_string());
		}
	}


	result
}

#[cfg(test)]

#[test]

fn test_common_chars() {
	let words = vec!["bella".to_string(),"label".to_string(),"roller".to_string()];
	let result = vec!["e".to_string(), "l".to_string(), "l".to_string()];
	assert_eq!(common_chars(words), result);

	let words = vec!["cool".to_string(),"lock".to_string(),"cook".to_string()];
	let result = vec!["c".to_string(), "o".to_string()];
	assert_eq!(common_chars(words), result);
}