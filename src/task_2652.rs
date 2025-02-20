/*Given a positive integer n, find the sum of all integers in the range [1, n] inclusive that are divisible by 3, 5, or 7.

Return an integer denoting the sum of all numbers in the given range satisfying the constraint.



Example 1:

Input: n = 7
Output: 21
Explanation: Numbers in the range [1, 7] that are divisible by 3, 5, or 7 are 3, 5, 6, 7. The sum of these numbers is 21.

Example 2:

Input: n = 10
Output: 40
Explanation: Numbers in the range [1, 10] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9, 10. The sum of these numbers is 40.

Example 3:

Input: n = 9
Output: 30
Explanation: Numbers in the range [1, 9] that are divisible by 3, 5, or 7 are 3, 5, 6, 7, 9. The sum of these numbers is 30.



Constraints:

1 <= n <= 103

*/

fn sum_of_multiples(n: i32) -> i32 {
	let mut sum = 0;
	for i in (3..=n).step_by(3) {
		sum+=i;
	}

	for i in (5..=n).step_by(5) {
		if i%3 != 0 {
			sum+=i;
		}
	}

	for i in (7..=n).step_by(7) {
		if i%5 != 0 && i%3 != 0 {
			sum+=i;
		}
	}
	sum
}

#[cfg(test)]
#[test]

fn test_sum_of_multiples() {
	let n = 7;
	let result = 21;
	assert_eq!(sum_of_multiples(n), result);

	let n = 10;
	let result = 40;
	assert_eq!(sum_of_multiples(n), result);

	let n = 9;
	let result = 30;
	assert_eq!(sum_of_multiples(n), result);

	let n = 15;
	let result = 81;
	assert_eq!(sum_of_multiples(n), result);
}