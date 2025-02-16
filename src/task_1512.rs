// Given an array of integers nums, return the number of good pairs.
//
// A pair (i, j) is called good if nums[i] == nums[j] and i < j.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,1,1,3]
// Output: 4
// Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
//
// Example 2:
//
// Input: nums = [1,1,1,1]
// Output: 6
// Explanation: Each pair in the array are good.
//
// Example 3:
//
// Input: nums = [1,2,3]
// Output: 0
//
// Constraints:
//
//     1 <= nums.length <= 100
//     1 <= nums[i] <= 100

use std::collections::{HashMap};
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut table: HashMap<i32, i32> = HashMap::new();
    for num in nums.iter() {
        *table.entry(*num).or_insert(0) += 1;
    }
    table.values().map(|&x| x * (x - 1) / 2).sum()
}

#[cfg(test)]

mod tests {
    use crate::task_1512::num_identical_pairs;
    #[test]
    fn test_num_identical_pairs() {
        let nums = vec![1,2,3,1,1,3];
        assert_eq!(num_identical_pairs(nums), 4);
        let nums = vec![1,1,1,1];
        assert_eq!(num_identical_pairs(nums), 6);
        let nums = vec![1,2,3];
        assert_eq!(num_identical_pairs(nums), 0);
    }
}