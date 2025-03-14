/*A ramp in an integer array nums is a pair (i, j) for which i < j and nums[i] <= nums[j]. The width of such a ramp is j - i.

Given an integer array nums, return the maximum width of a ramp in nums. If there is no ramp in nums, return 0.



Example 1:

Input: nums = [6,0,8,2,1,5]
Output: 4
Explanation: The maximum width ramp is achieved at (i, j) = (1, 5): nums[1] = 0 and nums[5] = 5.

Example 2:

Input: nums = [9,8,1,0,1,9,4,0,4,1]
Output: 7
Explanation: The maximum width ramp is achieved at (i, j) = (2, 9): nums[2] = 1 and nums[9] = 1.



Constraints:

2 <= nums.length <= 5 * 104
0 <= nums[i] <= 5 * 104

*/

fn max_width_ramp(nums: Vec<i32>) -> i32 {
	let mut stack = vec![];

	for (i, &x) in nums.iter().enumerate() {
		if stack.last().map(|&top| nums[top] >= x).unwrap_or(true) {
			stack.push(i);
		}
	}

	let mut ret = 0;

	for (i, &x) in nums.iter().enumerate().rev() {
		while stack.last().map(|&top| nums[top] <= x).unwrap_or(false) {
			ret = stack.pop().map_or(ret, |top| ret.max(i - top));
		}
	}

	ret as i32
}