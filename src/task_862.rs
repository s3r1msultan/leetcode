/*Given an integer array nums and an integer k, return the length of the shortest non-empty subarray of nums with a sum of at least k. If there is no such subarray, return -1.

A subarray is a contiguous part of an array.



Example 1:

Input: nums = [1], k = 1
Output: 1

Example 2:

Input: nums = [1,2], k = 4
Output: -1

Example 3:

Input: nums = [2,-1,2], k = 3
Output: 3



Constraints:

1 <= nums.length <= 105
-105 <= nums[i] <= 105
1 <= k <= 109

*/

pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::VecDeque;
    let mut shortest = i32::MAX;
    let mut queue = VecDeque::new();

    let mut prefix_sum = vec![0i64; nums.len() + 1];
    for i in 0..nums.len() {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
    }

    for i in 0..=nums.len() {
        while let Some(&j) = queue.front() {
            if prefix_sum[i] - prefix_sum[j] >= k as i64 {
                shortest = shortest.min((j - i + 1) as i32);
                queue.pop_front();
            } else {
                break;
            }
        }

        while let Some(&j) = queue.back() {
            if prefix_sum[j] < prefix_sum[i] {
                queue.pop_back();
            } else {
                break;
            }
        }

        queue.push_back(i);
    }
    if shortest == i32::MAX {
        return -1;
    }
    shortest
}