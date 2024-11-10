/*You are given an array nums of non-negative integers and an integer k.

An array is called special if the bitwise OR of all of its elements is at least k.

Return the length of the shortest special non-empty
subarray
of nums, or return -1 if no special subarray exists.



Example 1:

Input: nums = [1,2,3], k = 2

Output: 1

Explanation:

The subarray [3] has OR value of 3. Hence, we return 1.

Example 2:

Input: nums = [2,1,8], k = 10

Output: 3

Explanation:

The subarray [2,1,8] has OR value of 11. Hence, we return 3.

Example 3:

Input: nums = [1,2], k = 0

Output: 1

Explanation:

The subarray [1] has OR value of 1. Hence, we return 1.



Constraints:

1 <= nums.length <= 2 * 105
0 <= nums[i] <= 109
0 <= k <= 109

*/

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    fn update_bit_counts(bit_counts: &mut [i32; 32], num: i32, delta: i32) {
        for i in 0..32 {
            if num & 1 << i != 0 {
                bit_counts[i] += delta;
            }
        }
    }

    fn bits_to_num(bit_counts: &[i32; 32]) -> i32 {
        let mut num = 0;

        for i in 0..32 {
            if bit_counts[i] != 0 {
                num += 1 << i;
            }
        }

        num
    }

    let mut left = 0;
    let mut right = 0;
    let mut min_len = i32::MAX;

    let mut bit_counts = [0; 32];

    while right < nums.len() {
        update_bit_counts(&mut bit_counts, nums[right], 1);

        while left <= right && bits_to_num(&bit_counts) >= k {
            min_len = min_len.min((right - left + 1) as i32);
            update_bit_counts(&mut bit_counts, nums[left], -1);
            left += 1;
        }
        right += 1;
    }

    if min_len == i32::MAX {
        -1
    } else {
        min_len
    }
}