/*A sequence x1, x2, ..., xn is Fibonacci-like if:

n >= 3
xi + xi+1 == xi+2 for all i + 2 <= n

Given a strictly increasing array arr of positive integers forming a sequence, return the length of the longest Fibonacci-like subsequence of arr. If one does not exist, return 0.

A subsequence is derived from another sequence arr by deleting any number of elements (including none) from arr, without changing the order of the remaining elements. For example, [3, 5, 8] is a subsequence of [3, 4, 5, 6, 7, 8].



Example 1:

Input: arr = [1,2,3,4,5,6,7,8]
Output: 5
Explanation: The longest subsequence that is fibonacci-like: [1,2,3,5,8].

Example 2:

Input: arr = [1,3,7,11,12,14,18]
Output: 3
Explanation: The longest subsequence that is fibonacci-like: [1,11,12], [3,11,14] or [7,11,18].



Constraints:

3 <= arr.length <= 1000
1 <= arr[i] < arr[i + 1] <= 109

*/

pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let n = arr.len();
    let mut dp = vec![0; n];
    let mut max_len = 0;
    let mut map = HashMap::new();
    map.insert(arr[0], 0);
    for j in 1..n {
        map.insert(arr[j], j);
        for k in j +1..n {

            if let Some(&i) = map.get(&(arr[k] - arr[j])) {
                if arr[k] - arr[j] == arr[i] {
                    break;
                }
                dp[k] = 3.max(dp[i] + 1);
                max_len = max_len.max(dp[k]);
            }
        }
        map.insert(arr[j], j);
    }
    max_len
}