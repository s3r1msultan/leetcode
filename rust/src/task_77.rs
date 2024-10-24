/*Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].

You may return the answer in any order.



Example 1:

Input: n = 4, k = 2
Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
Explanation: There are 4 choose 2 = 6 total combinations.
Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.

Example 2:

Input: n = 1, k = 1
Output: [[1]]
Explanation: There is 1 choose 1 = 1 total combination.



Constraints:

1 <= n <= 20
1 <= k <= n

*/
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    fn backtrack(n: i32, start: i32, k: usize, result: &mut Vec<Vec<i32>>, current_combination: &mut Vec<i32>) {
        if current_combination.len() == k {
            result.push(current_combination.clone());
            return
        }
        for i in start..=n {
            current_combination.push(i);
            backtrack(n: i32, i + 1, k, result, current_combination);
            current_combination.pop();
        }
    }

    let mut result = vec![];
    backtrack(n, 1, k, &mut result, &mut vec![]);
    result
}