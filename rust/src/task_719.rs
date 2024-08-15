/*The distance of a pair of integers a and b is defined as the absolute difference between a and b.

Given an integer array nums and an integer k, return the kth smallest distance among all the pairs nums[i] and nums[j] where 0 <= i < j < nums.length.



Example 1:

Input: nums = [1,3,1], k = 1
Output: 0
Explanation: Here are all the pairs:
(1,3) -> 2
(1,1) -> 0
(3,1) -> 2
Then the 1st smallest distance pair is (1,1), and its distance is 0.

Example 2:

Input: nums = [1,1,1], k = 2
Output: 0

Example 3:

Input: nums = [1,6,1], k = 3
Output: 5



Constraints:

n == nums.length
2 <= n <= 104
0 <= nums[i] <= 106
1 <= k <= n * (n - 1) / 2

*/


fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
	let mut nums = nums;
	nums.sort();

	let mut left = 0;

	let mut right = nums[nums.len() - 1] - nums[0];

	while left < right {
		let mid = left + (right - left) / 2;
		let mut count = 0;
		let mut start = 0;
		for end in 0..nums.len() {
			while nums[end] - nums[start] > mid {
				start += 1;
			}
			count += end - start;
		}
		if count < k as usize {
			left = mid + 1;
		} else {
			right = mid;
		}
	}

	left
}