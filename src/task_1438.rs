// Given an array of integers nums and an integer limit, return the size of the longest non-empty subarray such that the absolute difference between any two elements of this subarray is less than or equal to limit.
//
//
//
// Example 1:
//
// Input: nums = [8,2,4,7], limit = 4
// Output: 2
// Explanation: All subarrays are:
// [8] with maximum absolute diff |8-8| = 0 <= 4.
// [8,2] with maximum absolute diff |8-2| = 6 > 4.
// [8,2,4] with maximum absolute diff |8-2| = 6 > 4.
// [8,2,4,7] with maximum absolute diff |8-2| = 6 > 4.
// [2] with maximum absolute diff |2-2| = 0 <= 4.
// [2,4] with maximum absolute diff |2-4| = 2 <= 4.
// [2,4,7] with maximum absolute diff |2-7| = 5 > 4.
// [4] with maximum absolute diff |4-4| = 0 <= 4.
// [4,7] with maximum absolute diff |4-7| = 3 <= 4.
// [7] with maximum absolute diff |7-7| = 0 <= 4.
// Therefore, the size of the longest subarray is 2.
//
// Example 2:
//
// Input: nums = [10,1,2,4,7,2], limit = 5
// Output: 4
// Explanation: The subarray [2,4,7,2] is the longest since the maximum absolute diff is |2-7| = 5 <= 5.
//
// Example 3:
//
// Input: nums = [4,2,2,2,4,4,2,2], limit = 0
// Output: 3
//
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// 1 <= nums[i] <= 109
// 0 <= limit <= 109
//

use std::collections::VecDeque;

pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
	let mut min_deque: VecDeque<usize> = VecDeque::new();
	let mut max_deque: VecDeque<usize> = VecDeque::new();
	let mut left = 0;
	let mut max_length = 0;
	for right in 0..nums.len() {
		while !min_deque.is_empty() && nums[*min_deque.back().unwrap()] > nums[right] {
			min_deque.pop_back();
		}

		while !max_deque.is_empty() && nums[*max_deque.back().unwrap()] < nums[right] {
			max_deque.pop_back();
		}

		min_deque.push_back(right);
		max_deque.push_back(right);

		while nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()] > limit {
			left += 1;
			if min_deque.front().unwrap() < &left {
				min_deque.pop_front();
			}

			if max_deque.front().unwrap() < &left {
				max_deque.pop_front();
			}
		}

		max_length = max_length.max(right - left + 1);
	}

	max_length as i32
}


#[cfg(test)]
#[test]
fn test_longest_subarray() {
	let nums = vec![8, 2, 4, 7];
	let limit = 4;
	let result = 2;
	assert_eq!(longest_subarray(nums, limit), result);

	let nums = vec![10, 1, 2, 4, 7, 2];
	let limit = 5;
	let result = 4;
	assert_eq!(longest_subarray(nums, limit), result);

	let nums = vec![4, 2, 2, 2, 4, 4, 2, 2];
	let limit = 0;
	let result = 3;
	assert_eq!(longest_subarray(nums, limit), result);
}