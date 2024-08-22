// The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.
//
// For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
//
// Given an integer num, return its complement.
//
//
//
// Example 1:
//
// Input: num = 5
// Output: 2
// Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.
//
// Example 2:
//
// Input: num = 1
// Output: 0
// Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.
//
//
//
// Constraints:
//
// 1 <= num < 231
fn find_complement(num: i32) -> i32 {
	fn to_binary(num: i32) -> String {
		let mut num = num;
		let mut binary_num = String::new();
		while num > 1 {
			binary_num.push(((num % 2) as u8 + '0' as u8) as char);
			num /= 2;
		}
		binary_num = binary_num.chars().rev().collect::<String>();
		let binary_num = num.to_string() + &binary_num;
		binary_num
	}
	let binary_num = to_binary(num);
	let mut complement = 0;

	for (i, digit) in binary_num.chars().rev().enumerate() {
		match digit {
			'0' => {
				complement += 2.pow(i as u32);
			}
			_ => {}
		}
	}


	complement
}