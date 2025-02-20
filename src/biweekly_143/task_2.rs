/*You are given an integer array nums and two integers k and numOperations.

You must perform an operation numOperations times on nums, where in each operation you:

Select an index i that was not selected in any previous operations.
Add an integer in the range [-k, k] to nums[i].

Return the maximum possible frequency of any element in nums after performing the operations.

The frequency of an element x is the number of times it occurs in the array.



Example 1:

Input: nums = [1,4,5], k = 1, numOperations = 2

Output: 2

Explanation:

We can achieve a maximum frequency of two by:

Adding 0 to nums[1]. nums becomes [1, 4, 5].
Adding -1 to nums[2]. nums becomes [1, 4, 4].

Example 2:

Input: nums = [5,11,20,20], k = 5, numOperations = 1

Output: 2

Explanation:

We can achieve a maximum frequency of two by:

Adding 0 to nums[1].



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 105
0 <= k <= 105
0 <= numOperations <= nums.length

Note: Please do not copy the description during the contest to maintain the integrity of your submissions.*/

/*pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut max_freq = 1;
    let mut left = 0;
    let mut sum_within_window = 0i64;
    for right in 1..nums.len() {
        sum_within_window += nums[right] as i64;

        while (right - left + 1) as i64 * nums[right] as i64 - sum_within_window > num_operations as i64 {
            sum_within_window -= nums[left] as i64;
            left += 1;
        }

        max_freq = max_freq.max((right - left + 1) as i32);
    }
    max_freq
}*/