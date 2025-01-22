/*You are given a 0-indexed array heights of positive integers, where heights[i] represents the height of the ith building.

If a person is in building i, they can move to any other building j if and only if i < j and heights[i] < heights[j].

You are also given another array queries where queries[i] = [ai, bi]. On the ith query, Alice is in building ai while Bob is in building bi.

Return an array ans where ans[i] is the index of the leftmost building where Alice and Bob can meet on the ith query. If Alice and Bob cannot move to a common building on query i, set ans[i] to -1.



Example 1:

Input: heights = [6,4,8,5,2,7], queries = [[0,1],[0,3],[2,4],[3,4],[2,2]]
Output: [2,5,-1,5,2]
Explanation: In the first query, Alice and Bob can move to building 2 since heights[0] < heights[2] and heights[1] < heights[2].
In the second query, Alice and Bob can move to building 5 since heights[0] < heights[5] and heights[3] < heights[5].
In the third query, Alice cannot meet Bob since Alice cannot move to any other building.
In the fourth query, Alice and Bob can move to building 5 since heights[3] < heights[5] and heights[4] < heights[5].
In the fifth query, Alice and Bob are already in the same building.
For ans[i] != -1, It can be shown that ans[i] is the leftmost building where Alice and Bob can meet.
For ans[i] == -1, It can be shown that there is no building where Alice and Bob can meet.

Example 2:

Input: heights = [5,3,8,2,6,1,4,6], queries = [[0,7],[3,5],[5,2],[3,0],[1,6]]
Output: [7,6,-1,4,6]
Explanation: In the first query, Alice can directly move to Bob's building since heights[0] < heights[7].
In the second query, Alice and Bob can move to building 6 since heights[3] < heights[6] and heights[5] < heights[6].
In the third query, Alice cannot meet Bob since Bob cannot move to any other building.
In the fourth query, Alice and Bob can move to building 4 since heights[3] < heights[4] and heights[0] < heights[4].
In the fifth query, Alice can directly move to Bob's building since heights[1] < heights[6].
For ans[i] != -1, It can be shown that ans[i] is the leftmost building where Alice and Bob can meet.
For ans[i] == -1, It can be shown that there is no building where Alice and Bob can meet.



Constraints:

1 <= heights.length <= 5 * 104
1 <= heights[i] <= 109
1 <= queries.length <= 5 * 104
queries[i] = [ai, bi]
0 <= ai, bi <= heights.length - 1

*/

pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    fn binary_search(height: i32, monotonic_stack: &Vec<(i32, usize)>) -> i32 {
        let mut start = 0;
        let mut end = monotonic_stack.len() - 1;
        let mut ans = -1;

        while start <= end {
            let mid = start + (end - start) / 2;
            if monotonic_stack[mid].0 > height {
                ans = ans.max(mid as i32);
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }

        ans
    }

    let n = queries.len();
    let mut queries = queries;
    let mut result = vec![-1; n];

    let mut monotonic_stack = vec![];
    let mut new_queries: Vec<Vec<(i32, usize)>> = vec![vec![]; n];

    for i in 0..queries.len() {
        let a = queries[i][0] as usize;
        let b = queries[i][1] as usize;
        if a > b { // here, we sort their indices in ascending order
            queries[i].swap(0, 1);
        }
        if heights[b] > heights[a] || a == b { // if the second index's height is greater than the previous one, we add it to the result vector. if they are equal then we add it to the result
            result[i] = b as i32;
        } else {
            new_queries[b].push((heights[a], i));
        }
    }

    for i in (0..heights.len()).rev() {
        let n = monotonic_stack.len() as i32;
        for &(height, j) in &new_queries[i] {
            let position = binary_search(height, &monotonic_stack);
            if position < n && position >= 0 {
                result[j] = monotonic_stack[position as usize].1 as i32;
            }
        }

        while !monotonic_stack.is_empty() && monotonic_stack.last().unwrap().0 == heights[i] {
            monotonic_stack.pop();
        }

        monotonic_stack.push((heights[i], i));
    }

    result
}