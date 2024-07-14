/*Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].

The test cases are generated so that the length of the output will never exceed 105.



Example 1:

Input: s = "3[a]2[bc]"
Output: "aaabcbc"

Example 2:

Input: s = "3[a2[c]]"
Output: "accaccacc"

Example 3:

Input: s = "2[abc]3[cd]ef"
Output: "abcabccdcdcdef"



Constraints:

1 <= s.length <= 30
s consists of lowercase English letters, digits, and square brackets '[]'.
s is guaranteed to be a valid input.
All the integers in s are in the range [1, 300].

*/

fn decode_string(s: String) -> String {
	let mut i = 0;
	let n = s.len();
	let mut stack: Vec<(String, i32)> = vec![(String::new(), 1)];
	let mut chars: Vec<char> = s.chars().collect();

	while i < n {
		if chars[i].is_digit(10) {
			let mut count = (chars[i] as u8 - '0' as u8) as i32;
			i += 1;
			while chars[i].is_digit(10) {
				count *= 10;
				count += (chars[i] as u8 - '0' as u8) as i32;
				i += 1;
			}
			i -= 1;
			if count == 0 {
				count = 1;
			}

			stack.last_mut().unwrap().1 = count;
		} else if chars[i] == '[' {
			stack.push((String::new(), 1));
		} else if chars[i] == ']' {
			let (s, _) = stack.pop().unwrap();
			let prelast = stack.last_mut().unwrap();
			prelast.0.push_str(&*s.repeat(prelast.1 as usize));
		} else {
			stack.last_mut().unwrap().0.push(chars[i]);
		}
		i += 1;
	}
	let (s, _) = stack.pop().unwrap();

	s
}

#[cfg(test)]
#[test]
fn test_decode_string() {
	let s = "3[a]2[bc]".to_string();
	let result = "aaabcbc".to_string();
	assert_eq!(decode_string(s), result);

	let s = "3[a2[c]]".to_string();
	let result = "accaccacc".to_string();
	assert_eq!(decode_string(s), result);

	let s = "2[abc]3[cd]ef".to_string();
	let result = "abcabccdcdcdef".to_string();
	assert_eq!(decode_string(s), result);
}