/*Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.



Example 1:

Input: nums = [3,2,3]
Output: 3

Example 2:

Input: nums = [2,2,1,1,1,2,2]
Output: 2



Constraints:

n == nums.length
1 <= n <= 5 * 104
-109 <= nums[i] <= 109


Follow-up: Could you solve the problem in linear time and in O(1) space?*/

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();
    for &num in &nums {
        *map.entry(num).or_insert(0) += 1;
    }

    map.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0
}