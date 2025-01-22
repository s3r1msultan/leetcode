/*Given an integer array nums and an integer k, find three non-overlapping subarrays of length k with maximum sum and return them.

Return the result as a list of indices representing the starting position of each interval (0-indexed). If there are multiple answers, return the lexicographically smallest one.



Example 1:

Input: nums = [1,2,1,2,6,7,5,1], k = 2
Output: [0,3,5]
Explanation: Subarrays [1, 2], [2, 6], [7, 5] correspond to the starting indices [0, 3, 5].
We could have also taken [2, 1], but an answer of [1, 3, 5] would be lexicographically larger.

Example 2:

Input: nums = [1,2,1,2,1,2,1,2,1], k = 2
Output: [0,2,4]



Constraints:

1 <= nums.length <= 2 * 104
1 <= nums[i] < 216
1 <= k <= floor(nums.length / 3)

*/

pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    fn dp(sums: &Vec<i32>, k: usize, i: usize, rem: usize, memo: &mut Vec<[i32; 4]>) -> i32 {
        if rem == 0 {
            return 0;
        }

        if i >= sums.len() {
            return if rem > 0 {
                i32::MIN
            } else {
                0
            };
        }

        if memo[i][rem] != -1 {
            return memo[i][rem];
        }

        let with_current = sums[i] + dp(sums, k, i + k, rem - 1, memo);
        let without_current = dp(sums, k, i + 1, rem, memo);

        memo[i][rem] = with_current.max(without_current);

        memo[i][rem]
    }

    fn dfs(sums: &Vec<i32>, k: usize, i: usize, rem: usize, memo: &mut Vec<[i32; 4]>, indices: &mut Vec<i32>) {
        if rem == 0 || i >= sums.len() {
            return;
        }

        let with_current = sums[i] + dp(sums, k, i + k, rem - 1, memo);
        let without_current = dp(sums, k, i + 1, rem, memo);

        if with_current >= without_current {
            indices.push(i as i32);
            dfs(sums, k, i + k, rem - 1, memo, indices);
        } else {
            dfs(sums, k, i + 1, rem, memo, indices);
        }
    }
    let n = nums.len();
    let k = k as usize;
    let mut sums = vec![];

    let mut sum = 0;
    for i in 0..k {
        sum += nums[i];
    }
    sums.push(sum);

    for i in k..n {
        sum += nums[i] - nums[i - k];
        sums.push(sum);
    }


    let mut memo = vec![[-1; 4]; n - k + 1];
    let mut indices = vec![];
    dp(&sums, k, 0, 3, &mut memo);
    dfs(&sums, k, 0, 3, &mut memo, &mut indices);
    indices
}