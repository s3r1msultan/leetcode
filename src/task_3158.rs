/*You are given an array nums, where each number in the array appears either once or twice.

Return the bitwise XOR of all the numbers that appear twice in the array, or 0 if no number appears twice.



Example 1:

Input: nums = [1,2,1,3]

Output: 1

Explanation:

The only number that appears twice in nums is 1.

Example 2:

Input: nums = [1,2,3]

Output: 0

Explanation:

No number appears twice in nums.

Example 3:

Input: nums = [1,2,2,1]

Output: 3

Explanation:

Numbers 1 and 2 appeared twice. 1 XOR 2 == 3.



Constraints:

1 <= nums.length <= 50
1 <= nums[i] <= 50
Each number in nums appears either once or twice.

*/

pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut mask: i64 = 0;
    let mut result = 0;
    for num in nums {
        if mask & 1 << (num - 1) != 0 {
            result ^= num;
        } else {
            mask |= 1 << (num - 1);
        }
    }
    result
}
// Using the HashSet
/*
pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut set = std::collections::HashSet::new();
    let mut result = vec![];

    for num in nums {
        if set.contains(&num) {
            result.push(num);
        } else {
            set.insert(num);
        }
    }

    result.iter().fold(0, |acc, curr| acc ^ curr)
}
*/