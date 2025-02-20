/*An array is considered special if every pair of its adjacent elements contains two numbers with different parity.

You are given an array of integer nums and a 2D integer matrix queries, where for queries[i] = [fromi, toi] your task is to check that
subarray
nums[fromi..toi] is special or not.

Return an array of booleans answer such that answer[i] is true if nums[fromi..toi] is special.



Example 1:

Input: nums = [3,4,1,2,6], queries = [[0,4]]

Output: [false]

Explanation:

The subarray is [3,4,1,2,6]. 2 and 6 are both even.

Example 2:

Input: nums = [4,3,1,6], queries = [[0,2],[2,3]]

Output: [false,true]

Explanation:

The subarray is [4,3,1]. 3 and 1 are both odd. So the answer to this query is false.
The subarray is [1,6]. There is only one pair: (1,6) and it contains numbers with different parity. So the answer to this query is true.



Constraints:

1 <= nums.length <= 105
1 <= nums[i] <= 105
1 <= queries.length <= 105
queries[i].length == 2
0 <= queries[i][0] <= queries[i][1] <= nums.length - 1

*/

pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = nums.len();
    let mut result = vec![];
    let mut prefix_sum = vec![0; n - 1];
    for i in 0..n - 1 {
        prefix_sum[i + 1] = prefix_sum[i] + if nums[i] % 2 == nums[i - 1] % 2 { 0 } else { 1 };
    }
    println!("{:?}", prefix_sum);
    for query in queries {
        let start = query[0] as usize;
        let end = query[1] as usize;
        result.push(prefix_sum[end] - prefix_sum[start] == end - start);
    }
    result
}