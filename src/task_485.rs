/*Given a binary array nums, return the maximum number of consecutive 1's in the array.



Example 1:

Input: nums = [1,1,0,1,1,1]
Output: 3
Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.

Example 2:

Input: nums = [1,0,1,1,0,1]
Output: 2



Constraints:

1 <= nums.length <= 105
nums[i] is either 0 or 1.

*/

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max = 0;

    let mut left = 0;
    for right in 0..n {
        if nums[right] == 0 {
            left = right + 1;
        } else {
            max = max.max(right - left + 1);
        }
    }

    max as i32
}