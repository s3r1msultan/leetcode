/*You are given a string num, representing a large integer. Return the largest-valued odd integer (as a string) that is a non-empty substring of num, or an empty string "" if no odd integer exists.

A substring is a contiguous sequence of characters within a string.



Example 1:

Input: num = "52"
Output: "5"
Explanation: The only non-empty substrings are "5", "2", and "52". "5" is the only odd number.

Example 2:

Input: num = "4206"
Output: ""
Explanation: There are no odd numbers in "4206".

Example 3:

Input: num = "35427"
Output: "35427"
Explanation: "35427" is already an odd number.



Constraints:

1 <= num.length <= 105
num only consists of digits and does not contain any leading zeros.

*/

fn largest_odd_number(num: String) -> String {
	let mut first_odd_index: usize;
	let mut max_odd = "".to_string();
	let digits = num.chars().rev().collect::<Vec<_>>();
	for (i, &digit) in digits.iter().enumerate() {
		let is_odd = (digit as u8 - '0' as u8) / 2;
		if is_odd {
			first_odd_index = i;
			break;
		}
	}
	max_odd = digits[first_odd_index..].iter().collect();
	max_odd
}