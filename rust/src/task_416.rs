/*Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.



Example 1:

Input: nums = [1,5,11,5]
Output: true
Explanation: The array can be partitioned as [1, 5, 5] and [11].

Example 2:

Input: nums = [1,2,3,5]
Output: false
Explanation: The array cannot be partitioned into equal sum subsets.



Constraints:

1 <= nums.length <= 200
1 <= nums[i] <= 100

*/

fn can_partition(nums: Vec<i32>) -> bool {
	let sum: i32 = nums.iter().sum();
	if sum % 2 == 1 {
		return false;
	}
	let n = nums.len();
	let half_sum = sum as usize / 2;
	let mut dp = vec![vec![false; half_sum + 1]; n + 1];
	dp.iter_mut().for_each(|x| *x[0] = true);
	for i in 1..=n {
		for j in 1..=half_sum {
			if nums[i - 1] > j as i32 {
				dp[i][j] = dp[i - 1][j];
			} else {
				dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i - 1] as usize];
			}
		}
	}
	dp[n][half_sum]
}