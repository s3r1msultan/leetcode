/*You are given a 0-indexed 2D integer array grid of size m x n. Each cell has one of two values:

0 represents an empty cell,
1 represents an obstacle that may be removed.

You can move up, down, left, or right from and to an empty cell.

Return the minimum number of obstacles to remove so you can move from the upper left corner (0, 0) to the lower right corner (m - 1, n - 1).



Example 1:

Input: grid = [[0,1,1],[1,1,0],[1,1,0]]
Output: 2
Explanation: We can remove the obstacles at (0, 1) and (0, 2) to create a path from (0, 0) to (2, 2).
It can be shown that we need to remove at least 2 obstacles, so we return 2.
Note that there may be other ways to remove 2 obstacles to create a path.

Example 2:

Input: grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]]
Output: 0
Explanation: We can move from (0, 0) to (2, 4) without removing any obstacles, so we return 0.



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 105
2 <= m * n <= 105
grid[i][j] is either 0 or 1.
grid[0][0] == grid[m - 1][n - 1] == 0

*/

pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let m = grid.len();
    let n = grid[0].len();

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut min_obstacles = vec![vec![i32::MAX; n]; m];

    let mut pq = BinaryHeap::new();

    min_obstacles[0][0] = grid[0][0];
    pq.push(Reverse((grid[0][0], 0, 0)));

    while let Some(Reverse((obstacles, row, col))) = pq.pop() {
        if row == m - 1 && col == n - 1 {
            return obstacles;
        }

        for &(dr, dc) in &directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_col >= 0 && new_row < m as isize && new_col < n as isize
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                let new_obstacles = obstacles + grid[new_row][new_col];

                if new_obstacles < min_obstacles[new_row][new_col] {
                    min_obstacles[new_row][new_col] = new_obstacles;
                    pq.push(Reverse((new_obstacles, new_row, new_col)));
                }
            }
        }
    }

    min_obstacles[m - 1][n - 1]
}