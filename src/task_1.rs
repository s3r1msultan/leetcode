// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// You can return the answer in any order.
//
//
//
// Example 1:
//
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
//
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
// Example 3:
//
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
//
//
// Constraints:
//
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.
//
//
// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut set = std::collections::HashMap::<i32, i32>::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = set.get(&(target - num)) {
            return vec![j, i as i32];
        }
        else {
            set.insert(num, i as i32);
        }
    }
    vec![]
}

#[cfg(test)]

#[test]

fn test_two_sum() {
    let nums: Vec<i32> = Vec::from([2, 7, 11, 15]);
    let target = 9;
    assert_eq!(two_sum(nums, target), vec![0,1]);

    let nums: Vec<i32> = Vec::from([3,2,4]);
    let target = 6;
    assert_eq!(two_sum(nums, target), vec![1,2]);

    let nums: Vec<i32> = Vec::from([3, 3]);
    let target = 6;
    assert_eq!(two_sum(nums, target), vec![0,1]);
}