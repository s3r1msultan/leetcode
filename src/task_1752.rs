/*Given an array nums, return true if the array was originally sorted in non-decreasing order, then rotated some number of positions (including zero). Otherwise, return false.

There may be duplicates in the original array.

Note: An array A rotated by x positions results in an array B of the same length such that A[i] == B[(i+x) % A.length], where % is the modulo operation.



Example 1:

Input: nums = [3,4,5,1,2]
Output: true
Explanation: [1,2,3,4,5] is the original sorted array.
You can rotate the array by x = 3 positions to begin on the the element of value 3: [3,4,5,1,2].

Example 2:

Input: nums = [2,1,3,4]
Output: false
Explanation: There is no sorted array once rotated that can make nums.

Example 3:

Input: nums = [1,2,3]
Output: true
Explanation: [1,2,3] is the original sorted array.
You can rotate the array by x = 0 positions (i.e. no rotation) to make nums.



Constraints:

1 <= nums.length <= 100
1 <= nums[i] <= 100

*/

// pub fn check(nums: Vec<i32>) -> bool {
//     let n = nums.len();
//     let mut sorted_nums = nums.clone();
//     sorted_nums.sort_unstable();
//
//     let mut rotated_nums = sorted_nums.clone();
//
//     for i in 0..n {
//         for j in 0..n {
//             rotated_nums[j] = sorted_nums[(j + i) % n];
//         }
//         if rotated_nums == nums {
//             return true;
//         }
//     }
//
//     false
// }

pub fn check(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut count = 0;
    for i in 0..n {
        if nums[i] > nums[(i + 1) % n] {
            count += 1;
        }
    }
    count < 2
}