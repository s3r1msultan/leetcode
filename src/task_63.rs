// You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.
//
// An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.
//
// Return the number of possible unique paths that the robot can take to reach the bottom-right corner.
//
// The testcases are generated so that the answer will be less than or equal to 2 * 109.
//
//
//
// Example 1:
//
// Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
// Output: 2
// Explanation: There is one obstacle in the middle of the 3x3 grid above.
// There are two ways to reach the bottom-right corner:
// 1. Right -> Right -> Down -> Down
// 2. Down -> Down -> Right -> Right
//
// Example 2:
//
// Input: obstacleGrid = [[0,1],[0,0]]
// Output: 1
//
//
//
// Constraints:
//
// m == obstacleGrid.length
// n == obstacleGrid[i].length
// 1 <= m, n <= 100
// obstacleGrid[i][j] is 0 or 1.
//

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    if obstacle_grid[0][0] == 1 {
        return 0;
    }
    let move_directions = [(0, 1), (1, 0)];
    let back_directions = [(0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }
    let n = obstacle_grid.len();
    let m = obstacle_grid[0].len();
    let mut seen = vec![vec![false; m]; n];
    let mut dp = vec![vec![0; m]; n];
    let mut queue = std::collections::VecDeque::new();
    dp[0][0] = 1;
    queue.push_back((0, 0));

    while let Some((i, j)) = queue.pop_front() {
        for &(di, dj) in &back_directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            dp[i][j] += dp[ni][nj];
        }

        for &(di, dj) in &move_directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if obstacle_grid[ni][nj] == 0 && !seen[ni][nj] {
                seen[ni][nj] = true;
                queue.push_back((ni, nj));
            }
        }
    }

    dp[n - 1][m - 1]
}