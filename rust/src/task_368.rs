/*Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:

answer[i] % answer[j] == 0, or
answer[j] % answer[i] == 0

If there are multiple solutions, return any of them.



Example 1:

Input: nums = [1,2,3]
Output: [1,2]
Explanation: [1,3] is also accepted.

Example 2:

Input: nums = [1,2,4,8]
Output: [1,2,4,8]



Constraints:

1 <= nums.length <= 1000
1 <= nums[i] <= 2 * 109
All the integers in nums are unique.

*/

pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
	let n = nums.len();

	let mut nums = nums;
	nums.sort_unstable();

	let mut dp: Vec<Vec<i32>> = vec![vec![]; n];
	for i in 0..n {
		dp[i].push(nums[i]);
	}


	for i in 1..n {
		for j in 0..i {
			if nums[i] % nums[j] == 0 && dp[i].len() < dp[j].len() + 1 {
				dp[i] = dp[j].clone();
				dp[i].push(nums[i]);
			}
		}
	}

	dp.iter().max_by(|a, b| b.len().cmp(&a.len())).unwrap().clone()
}