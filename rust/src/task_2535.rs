/*You are given a positive integer array nums.

The element sum is the sum of all the elements in nums.
The digit sum is the sum of all the digits (not necessarily distinct) that appear in nums.

Return the absolute difference between the element sum and digit sum of nums.

Note that the absolute difference between two integers x and y is defined as |x - y|.



Example 1:

Input: nums = [1,15,6,3]
Output: 9
Explanation:
The element sum of nums is 1 + 15 + 6 + 3 = 25.
The digit sum of nums is 1 + 1 + 5 + 6 + 3 = 16.
The absolute difference between the element sum and digit sum is |25 - 16| = 9.

Example 2:

Input: nums = [1,2,3,4]
Output: 0
Explanation:
The element sum of nums is 1 + 2 + 3 + 4 = 10.
The digit sum of nums is 1 + 2 + 3 + 4 = 10.
The absolute difference between the element sum and digit sum is |10 - 10| = 0.



Constraints:

1 <= nums.length <= 2000
1 <= nums[i] <= 2000

*/

fn difference_of_sum(nums: Vec<i32>) -> i32 {
	fn sum_digits(mut num:i32) -> i32 {
		let mut sum = 0;
		while num!=0 {
			sum+=num%10;
			num/=10;
		}
		sum
	}
	let mut element_sum = nums.iter().fold(0, |acc, &num| acc+num);
	let mut digit_sum = nums.iter().fold(0, |acc, &num| acc+sum_digits(num));
	element_sum.abs_diff(digit_sum) as i32
}

#[cfg(test)]
#[test]

fn test_difference_of_sum() {
	let nums = vec![1,15,6,3];
	let result = 9;
	assert_eq!(difference_of_sum(nums), result);

	let nums = vec![1,2,3,4];
	let result = 0;
	assert_eq!(difference_of_sum(nums), result);
}