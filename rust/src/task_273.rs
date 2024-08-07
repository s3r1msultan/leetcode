// Convert a non-negative integer num to its English words representation.
//
//
//
// Example 1:
//
// Input: num = 123
// Output: "One Hundred Twenty Three"
//
// Example 2:
//
// Input: num = 12345
// Output: "Twelve Thousand Three Hundred Forty Five"
//
// Example 3:
//
// Input: num = 1234567
// Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
//
//
//
// Constraints:
//
// 0 <= num <= 231 - 1
//


fn number_to_words(num: i32) -> String {
	if num == 0 {
		return "Zero".to_string();
	}
	let mut num = num;
	fn hundreds(num: i32, ones: &[&str; 20], tens: &[&str; 10]) -> String {
		let mut num = num;
		let mut result = "".to_string();

		if num / 100 != 0 {
			result.push(' ');
			result.push_str(ones[num / 100]);
			result.push_str(" Hundred ");
			num %= 100;
		}

		if num < 20 && num != 10 {
			result.push(' ');
			result.push_str(ones[num as usize]);
		} else {
			result.push(' ');
			result.push_str(tens[(num / 10) as usize]);
			if num % 10 != 0 {
				result.push(' ');
				result.push_str(ones[(num % 10) as usize]);
			}
		}
		result
	}
	let ones = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];
	let tens = ["", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];

	let mut result = "".to_string();
	if num / 10_i32.pow(9) != 0 {
		result.push_str(&hundreds(num / 10_i32.pow(9), &ones, &tens));
		result.push_str(" Billion ");
		num %= 10_i32.pow(9);
	}

	if num / 10_i32.pow(6) != 0 {
		result.push_str(&hundreds(num / 10_i32.pow(6), &ones, &tens));
		result.push_str(" Million ");
		num %= 10_i32.pow(6);
	}

	if num / 10_i32.pow(3) != 0 {
		result.push_str(&hundreds(num / 10_i32.pow(3), &ones, &tens));
		result.push_str(" Thousand ");
		num %= 10_i32.pow(3);
	}

	result.push_str(&hundreds(num, &ones, &tens));

	result.split_whitespace().collect::<Vec<_>>().join(" ")
}