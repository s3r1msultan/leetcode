/*Given an array nums of distinct integers, return all the possible
permutations
. You can return the answer in any order.



Example 1:

Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

Example 2:

Input: nums = [0,1]
Output: [[0,1],[1,0]]

Example 3:

Input: nums = [1]
Output: [[1]]



Constraints:

1 <= nums.length <= 6
-10 <= nums[i] <= 10
All the integers of nums are unique.

*/

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    fn backtrack(nums: &Vec<i32>, seen: &mut HashSet<i32>, result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }
        for i in 0..nums.len() {
            if !seen.contains(&nums[i]) {
                seen.insert(nums[i]);
                current.push(nums[i]);
                backtrack(nums, seen, result, current);
                current.pop();
                seen.remove(&nums[i]);
            }
        }
    }
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    let mut current = Vec::new();
    backtrack(&nums, &mut seen, &mut result, &mut current);
    result
}

