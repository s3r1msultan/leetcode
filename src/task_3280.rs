/*You are given a string date representing a Gregorian calendar date in the yyyy-mm-dd format.

date can be written in its binary representation obtained by converting year, month, and day to their binary representations without any leading zeroes and writing them down in year-month-day format.

Return the binary representation of date.



Example 1:

Input: date = "2080-02-29"

Output: "100000100000-10-11101"

Explanation:

100000100000, 10, and 11101 are the binary representations of 2080, 02, and 29 respectively.

Example 2:

Input: date = "1900-01-01"

Output: "11101101100-1-1"

Explanation:

11101101100, 1, and 1 are the binary representations of 1900, 1, and 1 respectively.



Constraints:

date.length == 10
date[4] == date[7] == '-', and all other date[i]'s are digits.
The input is generated such that date represents a valid Gregorian calendar date between Jan 1st, 1900 and Dec 31st, 2100 (both inclusive).

*/

fn convert_date_to_binary(date: String) -> String {
	fn to_binary(text: &str) -> String {
		let mut num = text.parse::<i32>().unwrap();
		let mut result = "".to_string();
		while num > 1 {
			result.insert(0, ('0' as u8 + (num % 2) as u8) as char);
			num /= 2;
		}
		result.insert(0, '1');
		result
	}
	let parts = date.split("-").map(|s| to_binary(s)).collect::<Vec<_>>();
	parts.join("-")
}