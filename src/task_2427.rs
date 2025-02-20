/*Given two positive integers a and b, return the number of common factors of a and b.

An integer x is a common factor of a and b if x divides both a and b.



Example 1:

Input: a = 12, b = 6
Output: 4
Explanation: The common factors of 12 and 6 are 1, 2, 3, 6.

Example 2:

Input: a = 25, b = 30
Output: 2
Explanation: The common factors of 25 and 30 are 1, 5.



Constraints:

1 <= a, b <= 1000

*/

fn common_factors(a: i32, b: i32) -> i32 {
	fn gcd(a: i32, b: i32) -> i32 {
		if b == 0 {
			return a;
		}

		return gcd(b, a % b);
	}
	let mut count = 1;
	let gcd = gcd(a, b);


	for i in 2..=gcd {
		if a % i == 0 && b % i == 0 {
			count += 1;
		}
	}

	count
}

#[cfg(test)]
#[test]
fn test_common_factors() {
	let a = 12;
	let b = 6;
	let result = 4;
	assert_eq!(common_factors(a, b), result);
	let a = 25;
	let b = 30;
	let result = 2;
	assert_eq!(common_factors(a, b), result);
}