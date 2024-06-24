/*You are given a binary array nums and an integer k.

A k-bit flip is choosing a subarray of length k from nums and simultaneously changing every 0 in the subarray to 1, and every 1 in the subarray to 0.

Return the minimum number of k-bit flips required so that there is no 0 in the array. If it is not possible, return -1.

A subarray is a contiguous part of an array.



Example 1:

Input: nums = [0,1,0], k = 1
Output: 2
Explanation: Flip nums[0], then flip nums[2].

Example 2:

Input: nums = [1,1,0], k = 2
Output: -1
Explanation: No matter how we flip subarrays of size 2, we cannot make the array become [1,1,1].

Example 3:

Input: nums = [0,0,0,1,0,1,1,0], k = 3
Output: 3
Explanation:
Flip nums[0],nums[1],nums[2]: nums becomes [1,1,1,1,0,1,1,0]
Flip nums[4],nums[5],nums[6]: nums becomes [1,1,1,1,1,0,0,0]
Flip nums[5],nums[6],nums[7]: nums becomes [1,1,1,1,1,1,1,1]



Constraints:

1 <= nums.length <= 105
1 <= k <= nums.length

*/

fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let mut total_flips = 0;
    let mut current_flips = 0;

    for i in 0..nums.len() {
        if i >= k as usize && nums[i - k as usize] == 2 {
            current_flips -= 1;
        }

        if current_flips % 2 == nums[i] {
            if i + k as usize > nums.len() {
                return -1;
            }
            nums[i] = 2;
            total_flips += 1;
            current_flips += 1;
        }
    }

    total_flips
}

#[cfg(test)]
#[test]
fn test_min_k_bit_flips() {
    let nums = vec![0, 1, 0];
    let k = 1;
    let result = 2;
    assert_eq!(min_k_bit_flips(nums, k), result);


    let nums = vec![1, 1, 0];
    let k = 2;
    let result = -1;
    assert_eq!(min_k_bit_flips(nums, k), result);


    let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
    let k = 3;
    let result = 3;
    assert_eq!(min_k_bit_flips(nums, k), result);
}