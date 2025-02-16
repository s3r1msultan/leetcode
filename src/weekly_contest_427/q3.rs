/*You are given an array of integers nums and an integer k.

Return the maximum sum of a non-empty subarray of nums, such that the size of the subarray is divisible by k.

A subarray is a contiguous non-empty sequence of elements within an array.



Example 1:

Input: nums = [1,2], k = 1

Output: 3

Explanation:

The subarray [1, 2] with sum 3 has length equal to 2 which is divisible by 1.

Example 2:

Input: nums = [-1,-2,-3,-4,-5], k = 4

Output: -10

Explanation:

The maximum sum subarray is [-1, -2, -3, -4] which has length equal to 4 which is divisible by 4.

Example 3:

Input: nums = [-5,1,2,-3,4], k = 2

Output: 4

Explanation:

The maximum sum subarray is [1, 2, -3, 4] which has length equal to 4 which is divisible by 2.



Constraints:

1 <= k <= nums.length <= 2 * 105
-109 <= nums[i] <= 109

*/

pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut min_prefix = vec![1_000_000_000_000_000i64; k];
    min_prefix[k - 1] = 0;

    let mut result = i64::MIN;

    let mut prefix_sum = 0;

    for i in 0..nums.len() {
        prefix_sum += nums[i] as i64;
        result = result.max(prefix_sum - min_prefix[i % k]);
        min_prefix[i % k] = min_prefix[i % k].min(prefix_sum);
    }

    result
}