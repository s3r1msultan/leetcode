/*

An integer array is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
    For example, [1,3,5,7,9], [7,7,7,7], and [3,-1,-5,-9] are arithmetic sequences.

Given an integer array nums, return the number of arithmetic subarrays of nums.

A subarray is a contiguous subsequence of the array.



Example 1:

Input: nums = [1,2,3,4]
Output: 3
Explanation: We have 3 arithmetic slices in nums: [1, 2, 3], [2, 3, 4] and [1,2,3,4] itself.

Example 2:

Input: nums = [1]
Output: 0



Constraints:

    1 <= nums.length <= 5000
    -1000 <= nums[i] <= 1000

*/

pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut result = 0;
    let mut left = 0;
    for right in 2..n {
        if nums[right] - nums[right - 1] != nums[left + 1] - nums[left] {
            left = right - 1;
        }

        result += (right - left - 1) as i32;
    }

    result
}
