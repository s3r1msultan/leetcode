/*Given the string s, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.



Example 1:

Input: s = "eleetminicoworoep"
Output: 13
Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.

Example 2:

Input: s = "leetcodeisgreat"
Output: 5
Explanation: The longest substring is "leetc" which contains two e's.

Example 3:

Input: s = "bcbcbc"
Output: 6
Explanation: In this case, the given string "bcbcbc" is the longest because all vowels: a, e, i, o and u appear zero times.



Constraints:

1 <= s.length <= 5 x 10^5
s contains only lowercase English letters.

*/

fn find_the_longest_substring(s: String) -> i32 {
	let mut vowels = vec![0; 5];
	let mut map = std::collections::HashMap::new();
	map.insert(0, -1);
	let mut result = 0;
	for (i, c) in s.chars().enumerate() {
		match c {
			'a' => vowels[0] += 1,
			'e' => vowels[1] += 1,
			'i' => vowels[2] += 1,
			'o' => vowels[3] += 1,
			'u' => vowels[4] += 1,
			_ => {}
		}
		let key = vowels.iter().fold(0, |acc, &x| acc * 2 + x % 2);
		if let Some(&j) = map.get(&key) {
			result = result.max(i as i32 - j);
		} else {
			map.insert(key, i as i32);
		}
	}
	result
}