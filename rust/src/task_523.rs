/*Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.

A good subarray is a subarray where:

its length is at least two, and
the sum of the elements of the subarray is a multiple of k.

Note that:

A subarray is a contiguous part of the array.
An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k.



Example 1:

Input: nums = [23,2,4,6,7], k = 6
Output: true
Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.

Example 2:

Input: nums = [23,2,6,4,7], k = 6
Output: true
Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42.
42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.

Example 3:

Input: nums = [23,2,6,4,7], k = 13
Output: false



Constraints:

1 <= nums.length <= 105
0 <= nums[i] <= 109
0 <= sum(nums[i]) <= 231 - 1
1 <= k <= 231 - 1

*/

use std::collections::HashSet;

fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
	use std::collections::HashSet;
	let mut seen = HashSet::new();
	let mut prefix = 0;
	for num in nums{
		let next = (num+prefix)%k;
		if seen.contains(&next) {
			return true;
		}
		seen.insert(prefix);
		prefix = next;
	}
	false
}

#[cfg(test)]
#[test]
fn test_check_subarray_sum() {
	let nums = vec![23,2,4,6,7];
	let k = 6;
	let result = true;
	assert_eq!(check_subarray_sum(nums.clone(),k), result);

	let k = 6;
	let result = true;
	assert_eq!(check_subarray_sum(nums.clone(),k), result);

	let k = 13;
	let result = false;
	assert_eq!(check_subarray_sum(nums.clone(),k), result);
}