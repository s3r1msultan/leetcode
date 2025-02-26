/*Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.



Example 1:

Input: nums = [4,3,2,7,8,2,3,1]
Output: [5,6]

Example 2:

Input: nums = [1,1]
Output: [2]



Constraints:

n == nums.length
1 <= n <= 105
1 <= nums[i] <= n



Follow up: Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.
*/

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let n = nums.len() as i32;
        let set = nums.into_iter().collect::<HashSet<_>>();
        let mut result = vec![];
        for num in 1..n {
            if !set.contains(&num) {
                result.push(num);
            }
        }
        result
}