// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.
//
// Note: You can only move either down or right at any point in time.
//
//
//
// Example 1:
//
// Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
// Output: 7
// Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
//
// Example 2:
//
// Input: grid = [[1,2,3],[4,5,6]]
// Output: 12
//
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 200
// 0 <= grid[i][j] <= 200
//

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut dp = vec![vec![i32::MAX; m]; n];
    dp[0][0] = grid[0][0];

    let mut queue = std::collections::VecDeque::new();

    while let Some((i, j)) = queue.pop_front() {
        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }


            let ni = ni as usize;
            let nj = nj as usize;

            if dp[i][j] + grid[ni][nj] < dp[ni][nj] {
                dp[ni][nj] = dp[i][j] + grid[ni][nj];
                queue.push_back((ni, nj));
            }
        }
    }

    dp[n - 1][m - 1]
}