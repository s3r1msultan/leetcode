/*You are given an integer array nums and an integer k.

In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.

Return the maximum number of operations you can perform on the array.



Example 1:

Input: nums = [1,2,3,4], k = 5
Output: 2
Explanation: Starting with nums = [1,2,3,4]:
- Remove numbers 1 and 4, then nums = [2,3]
- Remove numbers 2 and 3, then nums = []
There are no more pairs that sum up to 5, hence a total of 2 operations.

Example 2:

Input: nums = [3,1,3,4,3], k = 6
Output: 1
Explanation: Starting with nums = [3,1,3,4,3]:
- Remove the first two 3's, then nums = [1,4,3]
There are no more pairs that sum up to 6, hence a total of 1 operation.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 109
1 <= k <= 109

*/


use std::cmp::Ordering;

// fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
// 	use std::collections::HashMap;
// 	let mut visited: HashMap<i32, i32> = HashMap::new();
// 	let mut count = 0;
// 	for num in nums {
// 		if *visited.entry(k - num).or_insert(0) > 0 {
// 			count += 1;
// 			*visited.entry(k - num).or_insert(0) -= 1;
// 		} else {
// 			*visited.entry(num).or_insert(0) += 1;
// 		}
// 	}
// 	count
// }
//
fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
	use std::cmp::Ordering;

	let mut nums = nums;
	nums.sort_unstable();
	let mut count = 0;
	let mut left = 0;
	let mut right = nums.len() - 1;
	while left < right {
		let sum = nums[left] + nums[right];
		match sum.cmp(&k) {
			Ordering::Less => {
				left += 1;
			}
			Ordering::Equal => {
				right -= 1;
				left += 1;
				count += 1;
			}
			Ordering::Greater => {
				right -= 1;
			}
		}
	}
	count
}