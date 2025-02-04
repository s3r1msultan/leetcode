/*You are given an m x n grid where each cell can have one of three values:

0 representing an empty cell,
1 representing a fresh orange, or
2 representing a rotten orange.

Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.

Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.



Example 1:

Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
Output: 4

Example 2:

Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
Output: -1
Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.

Example 3:

Input: grid = [[0,2]]
Output: 0
Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 10
grid[i][j] is 0, 1, or 2.

*/

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut queue = VecDeque::new();
    let mut grid = grid;

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 2 {
                queue.push_back((i, j));
            }
        }
    }
    let mut minutes = 0;

    while !queue.is_empty() {
        let n = queue.len();
        for _ in 0..n {
            let (i, j) = queue.pop_front().unwrap();

            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if !is_valid(ni, nj, n as i32, m as i32) {
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if grid[ni][nj] == 1 {
                    grid[ni][nj] = 2;
                    queue.push_back((ni, nj));
                }
            }
        }
        minutes += 1;
    }

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                return -1;
            }
        }
    }

    minutes
}