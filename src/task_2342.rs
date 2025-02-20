/*You are given a 0-indexed array nums consisting of positive integers. You can choose two indices i and j, such that i != j, and the sum of digits of the number nums[i] is equal to that of nums[j].

Return the maximum value of nums[i] + nums[j] that you can obtain over all possible indices i and j that satisfy the conditions.



Example 1:

Input: nums = [18,43,36,13,7]
Output: 54
Explanation: The pairs (i, j) that satisfy the conditions are:
- (0, 2), both numbers have a sum of digits equal to 9, and their sum is 18 + 36 = 54.
- (1, 4), both numbers have a sum of digits equal to 7, and their sum is 43 + 7 = 50.
So the maximum sum that we can obtain is 54.

Example 2:

Input: nums = [10,12,19,14]
Output: -1
Explanation: There are no two numbers that satisfy the conditions, so we return -1.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 109

*/

pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut map = std::collections::HashMap::new();
    let mut max_sum = -1;

    fn get_digits_sum(n: i32) -> i32 {
        let mut n = n;
        let mut s = 0;
        while n != 0 {
            s += n % 10;
            n /= 10;
        }
        s
    }

    for i in 0..n {
        let mut curr_num = nums[i];
        let digits_sum = get_digits_sum(curr_num);
        if let Some(num) = map.get_mut(&digits_sum) {
            max_sum = max_sum.max(*num + curr_num);
            *num = *num.max(&mut curr_num);
        } else {
            map.insert(digits_sum, curr_num);
        }
    }

    max_sum
}