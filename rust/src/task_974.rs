/* Given an integer array nums and an integer k, return the number of non-empty subarrays that have a sum divisible by k.

A subarray is a contiguous part of an array.



Example 1:

Input: nums = [4,5,0,-2,-3,1], k = 5
Output: 7
Explanation: There are 7 subarrays with a sum divisible by k = 5:
[4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]

Example 2:

Input: nums = [5], k = 9
Output: 0



Constraints:

    1 <= nums.length <= 3 * 104
    -104 <= nums[i] <= 104
    2 <= k <= 104

 */

use std::collections::HashMap;

fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut prefix_sum = 0;
    let mut remainder_count = HashMap::new();
    remainder_count.insert(0, 1);

    let mut count = 0;

    for num in nums {
        prefix_sum += num;

        let mut remainder = prefix_sum%k;
        if remainder < 0 {
            remainder+=k;
        }


        if let Some(c) = remainder_count.get(&remainder) {
            count+=c;
        }


        *remainder_count.entry(remainder).or_insert(0)+=1;
    }
    count
}

#[cfg(test)]
#[test]

fn test_subarrays_div_by_k() {
    let nums = vec![4, 5, 0, -2, -3, 1];
    let k = 5;
    let result = 7;
    assert_eq!(subarrays_div_by_k(nums, k), result);
}
