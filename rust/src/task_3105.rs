/*You are given an array of integers nums. Return the length of the longest
subarray
of nums which is either
strictly increasing
or
strictly decreasing
.



Example 1:

Input: nums = [1,4,3,3,2]

Output: 2

Explanation:

The strictly increasing subarrays of nums are [1], [2], [3], [3], [4], and [1,4].

The strictly decreasing subarrays of nums are [1], [2], [3], [3], [4], [3,2], and [4,3].

Hence, we return 2.

Example 2:

Input: nums = [3,3,3,3]

Output: 1

Explanation:

The strictly increasing subarrays of nums are [3], [3], [3], and [3].

The strictly decreasing subarrays of nums are [3], [3], [3], and [3].

Hence, we return 1.

Example 3:

Input: nums = [3,2,1]

Output: 3

Explanation:

The strictly increasing subarrays of nums are [3], [2], and [1].

The strictly decreasing subarrays of nums are [3], [2], [1], [3,2], [2,1], and [3,2,1].

Hence, we return 3.



Constraints:

1 <= nums.length <= 50
1 <= nums[i] <= 50

*/
pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max_count = 0;

    let mut inc_count = 1;
    let mut dec_count = 1;

    for i in 1..n {
        if nums[i - 1] < nums[i] {
            inc_count += 1;
            dec_count = 1;
        } else if nums[i - 1] > nums[i] {
            dec_count += 1;
            inc_count = 1;
        } else {
            inc_count = 1;
            dec_count = 1;
        }
        max_count = max_count.max(inc_count).max(dec_count);
    }

    max_count
}