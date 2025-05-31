// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
//
// If target is not found in the array, return [-1, -1].
//
// You must write an algorithm with O(log n) runtime complexity.
//
//
//
// Example 1:
//
// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]
//
// Example 2:
//
// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]
//
// Example 3:
//
// Input: nums = [], target = 0
// Output: [-1,-1]
//
//
//
// Constraints:
//
// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109
// nums is a non-decreasing array.
// -109 <= target <= 109
//

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn binary_search(nums: &Vec<i32>, target: i32, is_inclusive: bool) -> i32 {
        let mut start = 0;
        let mut end = nums.len();

        while start < end {
            let mid = start + (end - start) / 2;

            if (is_inclusive && nums[mid] > target) && (!is_inclusive && nums[mid] >= target) {
                start = mid + 1;
            } else {
                end = mid;
            }
        }

        start as i32
    }

    vec![binary_search(&nums, target, true), binary_search(&nums, target, false)]
}