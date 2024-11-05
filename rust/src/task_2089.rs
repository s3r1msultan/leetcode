/*2089. Find Target Indices After Sorting Array
Easy
Topics
Companies
Hint

You are given a 0-indexed integer array nums and a target element target.

A target index is an index i such that nums[i] == target.

Return a list of the target indices of nums after sorting nums in non-decreasing order. If there are no target indices, return an empty list. The returned list must be sorted in increasing order.



Example 1:

Input: nums = [1,2,5,2,3], target = 2
Output: [1,2]
Explanation: After sorting, nums is [1,2,2,3,5].
The indices where nums[i] == 2 are 1 and 2.

Example 2:

Input: nums = [1,2,5,2,3], target = 3
Output: [3]
Explanation: After sorting, nums is [1,2,2,3,5].
The index where nums[i] == 3 is 3.

Example 3:

Input: nums = [1,2,5,2,3], target = 5
Output: [4]
Explanation: After sorting, nums is [1,2,2,3,5].
The index where nums[i] == 5 is 4.



Constraints:

1 <= nums.length <= 100
1 <= nums[i], target <= 100

*/

pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut leftmost = -1;
    while start <= end {
        let mid = start + (end - start) / 2;
        if !(mid >= 0 && mid < nums.len()) {
            break;
        }
        if nums[mid] == target {
            leftmost = mid as i32;
            end = mid - 1;
        } else if nums[mid] > target {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    if leftmost == -1 {
        return vec![];
    }

    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut rightmost = -1;
    while start <= end {
        let mid = start + (end - start) / 2;
        if !(mid >= 0 && mid < nums.len()) {
            break;
        }
        if nums[mid] == target {
            rightmost = mid as i32;
            start = mid + 1;
        } else if nums[mid] > target {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    (leftmost..=rightmost).into_iter().collect()
}