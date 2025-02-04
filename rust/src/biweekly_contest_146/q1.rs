/*Given an integer array nums, return the number of subarrays of length 3 such that the sum of the first and third numbers equals exactly half of the second number.

A subarray is a contiguous non-empty sequence of elements within an array.



Example 1:

Input: nums = [1,2,1,4,1]

Output: 1

Explanation:

Only the subarray [1,4,1] contains exactly 3 elements where the sum of the first and third numbers equals half the middle number.

Example 2:

Input: nums = [1,1,1]

Output: 0

Explanation:

[1,1,1] is the only subarray of length 3. However, its first and third numbers do not add to half the middle number.



Constraints:

3 <= nums.length <= 100
-100 <= nums[i] <= 100

Note: Please do not copy the description during the contest to maintain the integrity of your submissions.©leetcode*/


pub fn count_subarrays(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..nums.len() - 2 {
        let a = nums[i];
        let b = nums[i + 1];
        let c = nums[i + 2];
        if b % 2 == 0 && a + c == b / 2 {
            count += 1;
        }
    }
    count
}