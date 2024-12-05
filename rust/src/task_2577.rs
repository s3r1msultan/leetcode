/*You are given a m x n matrix grid consisting of non-negative integers where grid[row][col] represents the minimum time required to be able to visit the cell (row, col), which means you can visit the cell (row, col) only when the time you visit it is greater than or equal to grid[row][col].

You are standing in the top-left cell of the matrix in the 0th second, and you must move to any adjacent cell in the four directions: up, down, left, and right. Each move you make takes 1 second.

Return the minimum time required in which you can visit the bottom-right cell of the matrix. If you cannot visit the bottom-right cell, then return -1.



Example 1:

Input: grid = [[0,1,3,2],[5,1,2,5],[4,3,8,6]]
Output: 7
Explanation: One of the paths that we can take is the following:
- at t = 0, we are on the cell (0,0).
- at t = 1, we move to the cell (0,1). It is possible because grid[0][1] <= 1.
- at t = 2, we move to the cell (1,1). It is possible because grid[1][1] <= 2.
- at t = 3, we move to the cell (1,2). It is possible because grid[1][2] <= 3.
- at t = 4, we move to the cell (1,1). It is possible because grid[1][1] <= 4.
- at t = 5, we move to the cell (1,2). It is possible because grid[1][2] <= 5.
- at t = 6, we move to the cell (1,3). It is possible because grid[1][3] <= 6.
- at t = 7, we move to the cell (2,3). It is possible because grid[2][3] <= 7.
The final time is 7. It can be shown that it is the minimum time possible.

Example 2:

Input: grid = [[0,2,4],[3,2,1],[1,0,4]]
Output: -1
Explanation: There is no path from the top left to the bottom-right cell.



Constraints:

m == grid.length
n == grid[i].length
2 <= m, n <= 1000
4 <= m * n <= 105
0 <= grid[i][j] <= 105
grid[0][0] == 0
*/

pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let m = grid.len();
    let n = grid[0].len();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn is_valid(row: usize, col: usize, grid: &Vec<Vec<i32>>) -> bool {
        let directions = [(1, 0), (0, 1)];
        for (dr, dc) in directions {
            let new_row = row + dr;
            let new_col = col + dc;
            if grid[new_row][new_col] == 1 {
                return true;
            }
        }
        false
    }

    if !is_valid(0, 0, &grid) {
        return -1;
    }

    let mut heap = BinaryHeap::new();
    let mut minimum_time = vec![vec![i32::MAX; n]; m];

    heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((time, row, col))) = heap.pop() {
        if row == m - 1 && col == n - 1 {
            return time;
        }

        for (dr, dc) in &directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row < 0 || new_col < 0 || new_row >= m as isize || new_col >= n as isize {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            let mut new_time = grid[new_row][new_col];
            if time + 1 >= grid[new_row][new_col] {
                new_time = time + 1;
            } else {
                if (new_time - time) % 2 == 0 {
                    new_time += 1;
                }
            };
            if new_time < minimum_time[new_row][new_col] {
                minimum_time[new_row][new_col] = new_time;
                heap.push(Reverse((new_time, new_row, new_col)));
            }
        }
    }
    -1
}