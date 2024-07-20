/*Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

Notice that the solution set must not contain duplicate triplets.



Example 1:

Input: nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]
Explanation:
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
The distinct triplets are [-1,0,1] and [-1,-1,2].
Notice that the order of the output and the order of the triplets does not matter.

Example 2:

Input: nums = [0,1,1]
Output: []
Explanation: The only possible triplet does not sum up to 0.

Example 3:

Input: nums = [0,0,0]
Output: [[0,0,0]]
Explanation: The only possible triplet sums up to 0.



Constraints:

3 <= nums.length <= 3000
-105 <= nums[i] <= 105

*/


fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
	let m = row_sum.len();
	let n = col_sum.len();
	let mut matrix = vec![vec![0; n]; m];
	let mut row_sum = row_sum;
	let mut col_sum = col_sum;

	for i in 0..m {
		for j in 0..n {
			let min = row_sum[i].min(col_sum[j]);
			matrix[i][j] = min;
			row_sum[i] -= min;
			col_sum[j] -= min;
		}
	}

	matrix
}