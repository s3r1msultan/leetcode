/*Given an array of strings nums containing n unique binary strings each of length n, return a binary string of length n that does not appear in nums. If there are multiple answers, you may return any of them.



Example 1:

Input: nums = ["01","10"]
Output: "11"
Explanation: "11" does not appear in nums. "00" would also be correct.

Example 2:

Input: nums = ["00","01"]
Output: "11"
Explanation: "11" does not appear in nums. "10" would also be correct.

Example 3:

Input: nums = ["111","011","001"]
Output: "101"
Explanation: "101" does not appear in nums. "000", "010", "100", and "110" would also be correct.



Constraints:

n == nums.length
1 <= n <= 16
nums[i].length == n
nums[i] is either '0' or '1'.
All the strings of nums are unique.

*/



pub fn find_different_binary_string(nums: Vec<String>) -> String {
    use std::collections::HashSet;

    let mut set: HashSet<String> = nums.iter().collect();
    fn backtrack(result: &mut String, set: &HashSet<String>, temp: &mut String, n: usize) {
        if result.len() == n {
            if !set.contains(&temp) {
                *result = temp.clone();
                return;
            }
        }

        for digit in ['0', '1'] {
            temp.push(digit);
            backtrack(result, set, temp, n);
            temp.pop();
        }
    }
    let mut result = String::new();
    backtrack(&mut result, &set, &mut String::new(), set.len());
    result
}