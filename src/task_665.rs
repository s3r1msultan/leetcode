/*Given an array nums with n integers, your task is to check if it could become non-decreasing by modifying at most one element.

We define an array is non-decreasing if nums[i] <= nums[i + 1] holds for every i (0-based) such that (0 <= i <= n - 2).



Example 1:

Input: nums = [4,2,3]
Output: true
Explanation: You could modify the first 4 to 1 to get a non-decreasing array.

Example 2:

Input: nums = [4,2,1]
Output: false
Explanation: You cannot get a non-decreasing array by modifying at most one element.



Constraints:

n == nums.length
1 <= n <= 104
-105 <= nums[i] <= 105

*/

pub fn check_possibility(nums: Vec<i32>) -> bool {
    let n = nums.len();

    let mut non_increasing = vec![1; n];
    let mut non_decreasing = vec![1; n];

    for i in 1..n {
        non_decreasing[i] = non_decreasing[i-1] + if nums[i] >= nums[i-1] {
            1
        } else {
            0
        };
        non_increasing[i] = non_increasing[i-1] + if nums[i] <= nums[i-1] {
            1
        } else {
            0
        };
    }

    println!("{:?}", non_increasing);
    println!("{:?}", non_decreasing);

    for i in 0..n {
        if non_decreasing[i] + non_increasing[n-1] - non_increasing[i] >= n - 1 {
            return true;
        }
    }

    true
}