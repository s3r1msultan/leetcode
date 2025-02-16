/*Given a string s, rearrange the characters of s so that any two adjacent characters are not the same.

Return any possible rearrangement of s or return "" if not possible.



Example 1:

Input: s = "aab"
Output: "aba"

Example 2:

Input: s = "aaab"
Output: ""



Constraints:

1 <= s.length <= 500
s consists of lowercase English letters.

*/

pub fn reorganize_string(s: String) -> String {
	let mut heap = std::collections::BinaryHeap::new();
	let mut letter_occurrences = std::collections::HashMap::new();

	for ch in s.chars() {
		*letter_occurrences.entry(ch).or_insert(0) += 1;
	}

	for (ch, count) in letter_occurrences {
		heap.push((count, ch));
	}

	let mut result = vec![];

	while let Some((mut first_count, first_ch)) = heap.pop() {
		if result.last() == Some(&first_ch) {
			if let Some((second_count, second_ch)) = heap.pop() {
				result.push(second_ch);

				if second_count > 1 {
					heap.push((second_count - 1, second_ch));
				}
			} else {
				break;
			}
		} else {
			result.push(first_ch);
			first_count -= 1;
		}

		if first_count > 0 {
			heap.push((first_count, first_ch));
		}
	}


	if s.len() == result.len() {
		result.into_iter().collect()
	} else {
		"".to_string()
	}
}