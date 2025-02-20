/*Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.

Return the sorted array.



Example 1:

Input: nums = [1,1,2,2,2,3]
Output: [3,1,1,2,2,2]
Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.

Example 2:

Input: nums = [2,3,1,3,2]
Output: [1,3,3,2,2]
Explanation: '2' and '3' both have a frequency of 2, so they are sorted in decreasing order.

Example 3:

Input: nums = [-1,1,-6,4,5,-6,1,4,1]
Output: [5,-1,4,4,-6,-6,1,1,1]



Constraints:

1 <= nums.length <= 100
-100 <= nums[i] <= 100

*/

use std::cmp::Ordering;

fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
	use std::collections::HashMap;
	let mut frequencies = HashMap::new();
	for num in nums {
		*frequencies.entry(num).or_insert(0) += 1;
	}
	let mut map = frequencies.into_iter().collect::<Vec<_>>();
	map.sort_unstable_by(|a, b| {
		match a.1.cmp(&b.1) {
			Ordering::Equal => {
				a.0.cmp(&b.0)
			}
			_ => {
				a.1.cmp(&b.1)
			}
		}
	});
	let mut result = vec![];
	for (key, value) in map {
		for _ in 0..value {
			result.push(key);
		}
	}
	result
}