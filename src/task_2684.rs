/*You are given a 0-indexed m x n matrix grid consisting of positive integers.

You can start at any cell in the first column of the matrix, and traverse the grid in the following way:

From a cell (row, col), you can move to any of the cells: (row - 1, col + 1), (row, col + 1) and (row + 1, col + 1) such that the value of the cell you move to, should be strictly bigger than the value of the current cell.

Return the maximum number of moves that you can perform.



Example 1:

Input: grid = [[2,4,3,5],[5,4,9,3],[3,4,2,11],[10,9,13,15]]
Output: 3
Explanation: We can start at the cell (0, 0) and make the following moves:
- (0, 0) -> (0, 1).
- (0, 1) -> (1, 2).
- (1, 2) -> (2, 3).
It can be shown that it is the maximum number of moves that can be made.

Example 2:


Input: grid = [[3,2,4],[2,1,9],[1,1,7]]
Output: 0
Explanation: Starting from any cell in the first column we cannot perform any moves.



Constraints:

m == grid.length
n == grid[i].length
2 <= m, n <= 1000
4 <= m * n <= 105
1 <= grid[i][j] <= 106

*/

pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &Vec<Vec<i32>>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i][j] != -1 {
            return dp[i][j];
        }

        let n = grid.len();
        let m = grid[0].len();

        let mut max_moves = 0;

        for row in [-1, 0, 1] {
            let new_row = i + row as usize;
            let new_col = j + 1;

            if new_col >= 0 && new_col < m && new_row >= 0 && new_row < n && grid[i][j] < grid[new_row][new_col] {
                max_moves = max_moves.max(dfs(grid, new_row, new_col, dp));
            }
        }

        dp[i][j] = max_moves;

        max_moves
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut dp = vec![vec![-1; m]; n];
    let mut max_moves = 0;
    for i in 0..n {
        max_moves = max_moves.max(dfs(&grid, i, 0, &mut dp));
    }

    max_moves
}