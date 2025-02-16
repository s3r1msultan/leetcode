/*You are given an integer array nums consisting of n elements, and an integer k.

Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.



Example 1:

Input: nums = [1,12,-5,-6,50,3], k = 4
Output: 12.75000
Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75

Example 2:

Input: nums = [5], k = 1
Output: 5.00000



Constraints:

n == nums.length
1 <= k <= n <= 105
-104 <= nums[i] <= 104

*/

fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
	let k = k as usize;
	let mut sum = 0;
	for &num in nums[..k].iter() {
		sum += num;
	}
	let mut max_avg = sum as f64 / k as f64;
	for i in k..nums.len() {
		sum -= nums[i - k];
		sum += nums[i];
		max_avg = max_avg.max(sum as f64 / k as f64);
	}
	max_avg
}