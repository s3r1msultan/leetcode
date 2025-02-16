/*Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.



Example 1:

Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
Output: 6
Explanation: [1,1,1,0,0,1,1,1,1,1,1]
Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.

Example 2:

Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
Output: 10
Explanation: [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.



Constraints:

1 <= nums.length <= 105
nums[i] is either 0 or 1.
0 <= k <= nums.length

*/

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut max = 0;
    let mut window_start = 0;
    let mut count_zeros = 0;
    for (window_end, &num) in nums.iter().enumerate() {
        if num == 0 {
            count_zeros += 1;
        }

        while count_zeros == k + 1 {
            if nums[window_start] == 0 {
                count_zeros -= 1;
            }
            window_start += 1;
        }

        max = max.max((window_end - window_start) as i32);
    }

    max
}