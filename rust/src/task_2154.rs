/*You are given an array of integers nums. You are also given an integer original which is the first number that needs to be searched for in nums.

You then do the following steps:

If original is found in nums, multiply it by two (i.e., set original = 2 * original).
Otherwise, stop the process.
Repeat this process with the new number as long as you keep finding the number.

Return the final value of original.



Example 1:

Input: nums = [5,3,6,1,12], original = 3
Output: 24
Explanation:
- 3 is found in nums. 3 is multiplied by 2 to obtain 6.
- 6 is found in nums. 6 is multiplied by 2 to obtain 12.
- 12 is found in nums. 12 is multiplied by 2 to obtain 24.
- 24 is not found in nums. Thus, 24 is returned.

Example 2:

Input: nums = [2,7,9], original = 4
Output: 4
Explanation:
- 4 is not found in nums. Thus, 4 is returned.



Constraints:

1 <= nums.length <= 1000
1 <= nums[i], original <= 1000

*/

pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    use std::collections::HashSet;
    
    let mut set = nums.into_iter().collect::<HashSet<i32>>();
    let mut original = original;

    while set.contains(&original) {
        original *= 2;
    }

    original
}