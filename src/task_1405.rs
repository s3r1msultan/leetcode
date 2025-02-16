/*A string s is called happy if it satisfies the following conditions:

s only contains the letters 'a', 'b', and 'c'.
s does not contain any of "aaa", "bbb", or "ccc" as a substring.
s contains at most a occurrences of the letter 'a'.
s contains at most b occurrences of the letter 'b'.
s contains at most c occurrences of the letter 'c'.

Given three integers a, b, and c, return the longest possible happy string. If there are multiple longest happy strings, return any of them. If there is no such string, return the empty string "".

A substring is a contiguous sequence of characters within a string.



Example 1:

Input: a = 1, b = 1, c = 7
Output: "ccaccbcc"
Explanation: "ccbccacc" would also be a correct answer.

Example 2:

Input: a = 7, b = 1, c = 0
Output: "aabaa"
Explanation: It is the only correct answer in this case.



Constraints:

0 <= a, b, c <= 100
a + b + c > 0

*/

pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
	let mut heap = std::collections::BinaryHeap::new();
	heap.push((a, 'a'));
	heap.push((b, 'b'));
	heap.push((c, 'c'));

	let mut result = vec![];

	while let Some((mut first_count, first_ch)) = heap.pop() {
		if result.len() >= 2 && result.get(result.len() - 1) == Some(&first_ch) && result.get(result.len() - 2) == Some(&first_ch) {
			if heap.is_empty() {
				break;
			}

			if let Some((second_count, second_ch)) = heap.pop() {
				result.push(second_ch);

				if second_count > 1 {
					heap.push((second_count - 1, second_ch));
				}
			}
		} else {
			first_count -= 1;
			result.push(first_ch);
		}
		if first_count > 0 {
			heap.push((first_count, first_ch));
		}
	}

	result.iter().collect()
}