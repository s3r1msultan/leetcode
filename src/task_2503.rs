/*You are given an m x n integer matrix grid and an array queries of size k.

Find an array answer of size k such that for each integer queries[i] you start in the top left cell of the matrix and repeat the following process:

If queries[i] is strictly greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell, and you can move to any adjacent cell in all 4 directions: up, down, left, and right.
Otherwise, you do not get any points, and you end this process.

After the process, answer[i] is the maximum number of points you can get. Note that for each query you are allowed to visit the same cell multiple times.

Return the resulting array answer.



Example 1:

Input: grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
Output: [5,8,1]
Explanation: The diagrams above show which cells we visit to get points for each query.

Example 2:

Input: grid = [[5,2,1],[1,1,2]], queries = [3]
Output: [0]
Explanation: We can not get any points because the value of the top left cell is already greater than or equal to 3.



Constraints:

m == grid.length
n == grid[i].length
2 <= m, n <= 1000
4 <= m * n <= 105
k == queries.length
1 <= k <= 104
1 <= grid[i][j], queries[i] <= 106

*/
pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    use std::collections::{BinaryHeap};
    use std::cmp::Reverse;
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && i < n && j >= 0 && j < m
    }

    let mut q_heap = BinaryHeap::new();

    for (i, &query) in queries.iter().enumerate() {
        q_heap.push(Reverse((query, i)));
    }

    let mut count = 0;

    let mut min_heap = BinaryHeap::new();
    let mut is_visited = vec![vec![false; m]; n];
    min_heap.push(Reverse((grid[0][0], 0, 0)));
    is_visited[0][0] = true;

    let mut result = vec![0; queries.len()];
    let (mut query, mut k) = q_heap.pop().unwrap().0;

    let mut heap_is_empty = false;

    while let Some(Reverse((val, i, j))) = min_heap.pop() {
        while val >= query {
            result[k] = count;
            if let Some(Reverse((new_query, new_k))) = q_heap.pop() {
                query = new_query;
                k = new_k;
            } else {
                heap_is_empty = true;
                break;
            }
        }

        if heap_is_empty {
            break;
        }

        count += 1;

        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if !is_visited[ni][nj] {
                is_visited[ni][nj] = true;
                min_heap.push(Reverse((grid[ni][nj], ni, nj)));
            }
        }
    }
    result[k] = count;

    while let Some(Reverse((_, new_k))) = q_heap.pop() {
        result[new_k] = count;
    }

    result
}