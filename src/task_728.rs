/*A self-dividing number is a number that is divisible by every digit it contains.

For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.

A self-dividing number is not allowed to contain the digit zero.

Given two integers left and right, return a list of all the self-dividing numbers in the range [left, right].



Example 1:

Input: left = 1, right = 22
Output: [1,2,3,4,5,6,7,8,9,11,12,15,22]

Example 2:

Input: left = 47, right = 85
Output: [48,55,66,77]



Constraints:

1 <= left <= right <= 104

*/


fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
	let mut dividing_numbers = vec![];
	for mut num in left..=right {
		let mut remainder = 0;
		let mut cloned = num;
		while cloned != 0 {
			if cloned % 10 == 0 {
				remainder = 1;
				break;
			}
			remainder += num % (cloned % 10);
			cloned /= 10;
		}
		if remainder == 0 {
			dividing_numbers.push(num);
		}
	}
	dividing_numbers
}

#[cfg(test)]
#[test]
fn test_self_dividing_numbers() {
	let left = 1;
	let right = 22;
	let result = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
	assert_eq!(self_dividing_numbers(left, right), result);

	let left = 47;
	let right = 85;
	let result = vec![48, 55, 66, 77];
	assert_eq!(self_dividing_numbers(left, right), result);
}