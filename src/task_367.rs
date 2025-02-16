/*Given a positive integer num, return true if num is a perfect square or false otherwise.

A perfect square is an integer that is the square of an integer. In other words, it is the product of some integer with itself.

You must not use any built-in library function, such as sqrt.



Example 1:

Input: num = 16
Output: true
Explanation: We return true because 4 * 4 = 16 and 4 is an integer.

Example 2:

Input: num = 14
Output: false
Explanation: We return false because 3.742 * 3.742 = 14 and 3.742 is not an integer.



Constraints:

1 <= num <= 231 - 1

*/

fn is_perfect_square(num: i32) -> bool {
	let mut start = 1i64;
	let mut end = num as i64;
	let mut mid = start + (end - start) / 2;
	while start <= end {
		mid = start + (end - start) / 2;
		if mid * mid == num as i64 {
			return true;
		} else if mid * mid < num as i64 {
			start = mid + 1;
		} else {
			end = mid - 1;
		}
	}

	end * end == num as i64
}

#[cfg(test)]
#[test]
fn test_is_perfect_square() {
	let num = 16;
	let result = true;
	assert_eq!(is_perfect_square(num), result);

	let num = 14;
	let result = false;
	assert_eq!(is_perfect_square(num), result);
}