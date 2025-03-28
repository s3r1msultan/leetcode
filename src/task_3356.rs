/*You are given an integer array nums of length n and a 2D array queries where queries[i] = [li, ri, vali].

Each queries[i] represents the following action on nums:

Decrement the value at each index in the range [li, ri] in nums by at most vali.
The amount by which each value is decremented can be chosen independently for each index.

A Zero Array is an array with all its elements equal to 0.

Return the minimum possible non-negative value of k, such that after processing the first k queries in sequence, nums becomes a Zero Array. If no such k exists, return -1.



Example 1:

Input: nums = [2,0,2], queries = [[0,2,1],[0,2,1],[1,1,3]]

Output: 2

Explanation:

For i = 0 (l = 0, r = 2, val = 1):
Decrement values at indices [0, 1, 2] by [1, 0, 1] respectively.
The array will become [1, 0, 1].
For i = 1 (l = 0, r = 2, val = 1):
Decrement values at indices [0, 1, 2] by [1, 0, 1] respectively.
The array will become [0, 0, 0], which is a Zero Array. Therefore, the minimum value of k is 2.

Example 2:

Input: nums = [4,3,2,1], queries = [[1,3,2],[0,2,1]]

Output: -1

Explanation:

For i = 0 (l = 1, r = 3, val = 2):
Decrement values at indices [1, 2, 3] by [2, 2, 1] respectively.
The array will become [4, 1, 0, 0].
For i = 1 (l = 0, r = 2, val = 1):
Decrement values at indices [0, 1, 2] by [1, 1, 0] respectively.
The array will become [3, 0, 0, 0], which is not a Zero Array.



Constraints:

1 <= nums.length <= 105
0 <= nums[i] <= 5 * 105
1 <= queries.length <= 105
queries[i].length == 3
0 <= li <= ri < nums.length
1 <= vali <= 5

*/

pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    fn is_possible(k: usize, nums: &Vec<i32>, queries: &Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut decrements = vec![0; n + 1];

        for i in 0..k {
            let start = queries[i][0] as usize;
            let end = queries[i][1] as usize;
            let val = queries[i][2];

            decrements[start] += val;
            decrements[end + 1] -= val;
        }

        for i in 1..=n {
            decrements[i] += decrements[i - 1];
        }

        for i in 0..n {
            if nums[i] > decrements[i] {
                return false;
            }
        }

        true
    }
    let q = queries.len();

    let mut start = 0;
    let mut end = q;
    let mut result = -1;

    while start <= end {
        let mid = start + (end - start) / 2;

        if is_possible(mid, &nums, &queries) {
            result = mid as i32;
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }

    result
}