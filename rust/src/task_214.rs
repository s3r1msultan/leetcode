/*You are given a string s. You can convert s to a
palindrome
by adding characters in front of it.

Return the shortest palindrome you can find by performing this transformation.



Example 1:

Input: s = "aacecaaa"
Output: "aaacecaaa"

Example 2:

Input: s = "abcd"
Output: "dcbabcd"



Constraints:

0 <= s.length <= 5 * 104
s consists of lowercase English letters only.

*/

fn shortest_palindrome(s: String) -> String {
	let mut s = s;
	let n = s.len();
	if n == 0 {
		return "".to_string();
	}


	fn is_palindrome(s: &str) -> bool {
		let mid = s.len() / 2;
		if s.len() % 2 == 0 {
			s[..mid].eq(&s[mid..].chars().rev().collect::<String>())
		} else {
			s[..mid].eq(&s[mid + 1..].chars().rev().collect::<String>())
		}
	}
	let mut first_pointer = 0;
	let mut second_pointer = n - 1;
	while first_pointer < second_pointer {
		println!("{}", &s[first_pointer..=second_pointer]);
		if !is_palindrome(&s[first_pointer..=second_pointer]) {
			second_pointer -= 1;
		} else {
			break;
		}
	}

	s[second_pointer + 1..].chars().rev().collect::<String>() + &s
}