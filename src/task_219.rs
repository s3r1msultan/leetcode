/*Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.



Example 1:

Input: nums = [1,2,3,1], k = 3
Output: true

Example 2:

Input: nums = [1,0,1,1], k = 1
Output: true

Example 3:

Input: nums = [1,2,3,1,2,3], k = 2
Output: false



Constraints:

1 <= nums.length <= 105
-109 <= nums[i] <= 109
0 <= k <= 105

*/

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let mut map = std::collections::HashSet::new();
    let mut left = 0;
    for right in 0..n {
        if map.contains(&nums[right]) {
            return true;
        }
        map.insert(nums[right]);
        if right - left == k as usize {
            map.remove(&nums[left]);
            left += 1;
        }
    }
    false
}