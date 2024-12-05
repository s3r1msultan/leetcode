/*Given a 0-indexed integer array nums of size n and two integers lower and upper, return the number of fair pairs.

A pair (i, j) is fair if:

0 <= i < j < n, and
lower <= nums[i] + nums[j] <= upper



Example 1:

Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
Output: 6
Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).

Example 2:

Input: nums = [1,7,9,2,5], lower = 11, upper = 11
Output: 1
Explanation: There is a single fair pair: (2,3).



Constraints:

1 <= nums.length <= 105
nums.length == n
-109 <= nums[i] <= 109
-109 <= lower <= upper <= 109

*/

pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut count = 0i64;
    for (i, &n) in nums.iter().enumerate() {
        let left = nums.partition_point(|&x| x + n < lower).max(i + 1);
        let right = nums.partition_point(|&x| x + n <= upper).max(i + 1);
        count += 0.max(right - left) as i64
    }
    count
}