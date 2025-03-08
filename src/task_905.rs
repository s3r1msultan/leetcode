/*Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.

Return any array that satisfies this condition.



Example 1:

Input: nums = [3,1,2,4]
Output: [2,4,3,1]
Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.

Example 2:

Input: nums = [0]
Output: [0]



Constraints:

1 <= nums.length <= 5000
0 <= nums[i] <= 5000

*/


pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut even_nums = vec![];
    let mut odd_nums = vec![];

    for num in nums {
        if num % 2 == 0 {
            even_nums.push(num);
        } else {
            odd_nums.push(num);
        }
    }

    vec![even_nums, odd_nums].concat()
}