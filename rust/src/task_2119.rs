/*Reversing an integer means to reverse all its digits.

For example, reversing 2021 gives 1202. Reversing 12300 gives 321 as the leading zeros are not retained.

Given an integer num, reverse num to get reversed1, then reverse reversed1 to get reversed2. Return true if reversed2 equals num. Otherwise return false.



Example 1:

Input: num = 526
Output: true
Explanation: Reverse num to get 625, then reverse 625 to get 526, which equals num.

Example 2:

Input: num = 1800
Output: false
Explanation: Reverse num to get 81, then reverse 81 to get 18, which does not equal num.

Example 3:

Input: num = 0
Output: true
Explanation: Reverse num to get 0, then reverse 0 to get 0, which equals num.



Constraints:

0 <= num <= 106

*/

fn is_same_after_reversals(num: i32) -> bool {
	// let mut cloned = num;
	// let mut reversed = 0;
	// while cloned != 0 {
	// 	reversed *=10;
	// 	reversed += cloned %10;
	// 	cloned /=10;
	// }
	// while reversed != 0 {
	// 	cloned*=10;
	// 	cloned+=reversed%10;
	// 	reversed/=10;
	// }
	// cloned == num

	num%10!=0 || num == 0
}

#[cfg(test)]
#[test]

fn test_is_same_after_reversals() {
	let num = 1800;
	let result = false;
	assert_eq!(is_same_after_reversals(num), result);

	let num = 526;
	let result = true;
	assert_eq!(is_same_after_reversals(num), result);

	let num = 0;
	let result = true;
	assert_eq!(is_same_after_reversals(num), result);
}