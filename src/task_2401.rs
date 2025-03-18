/*You are given an array nums consisting of positive integers.

We call a subarray of nums nice if the bitwise AND of every pair of elements that are in different positions in the subarray is equal to 0.

Return the length of the longest nice subarray.

A subarray is a contiguous part of an array.

Note that subarrays of length 1 are always considered nice.



Example 1:

Input: nums = [1,3,8,48,10]
Output: 3
Explanation: The longest nice subarray is [3,8,48]. This subarray satisfies the conditions:
- 3 AND 8 = 0.
- 3 AND 48 = 0.
- 8 AND 48 = 0.
It can be proven that no longer nice subarray can be obtained, so we return 3.

Example 2:

Input: nums = [3,1,5,11,13]
Output: 1
Explanation: The length of the longest nice subarray is 1. Any subarray of length 1 can be chosen.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 109

*/

/*pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    fn is_nice_subarray(mask: i32, num: i32) -> bool {
        for i in 0..31 {
            if (mask >> i) & 1 == 1 && (num >> i) & 1 == 1 {
                return false;
            }
        }
        true
    }

    let n = nums.len();
    let mut bitwise_or = 0;
    let mut max = 1;

    let mut left = 0;
    for right in 0..n {
        while left < right && !is_nice_subarray(bitwise_or, nums[right]) {
            bitwise_or ^= nums[left];
            left += 1;
        }
        bitwise_or |= nums[right];
        max = max.max(right - left + 1);
    }

    max as i32
}*/

pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut bitwise_or = 0;
    let mut max = 1;

    let mut left = 0;
    for right in 0..n {
        while bitwise_or & nums[right] != 0 {
            bitwise_or ^= nums[left];
            left += 1;
        }
        bitwise_or |= nums[right];
        max = max.max(right - left + 1);
    }

    max as i32
}