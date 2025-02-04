/*You are given an m x n integer matrix grid, and three integers row, col, and color. Each value in the grid represents the color of the grid square at that location.

Two squares are called adjacent if they are next to each other in any of the 4 directions.

Two squares belong to the same connected component if they have the same color and they are adjacent.

The border of a connected component is all the squares in the connected component that are either adjacent to (at least) a square not in the component, or on the boundary of the grid (the first or last row or column).

You should color the border of the connected component that contains the square grid[row][col] with color.

Return the final grid.



Example 1:

Input: grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
Output: [[3,3],[3,2]]

Example 2:

Input: grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
Output: [[1,3,3],[2,3,3]]

Example 3:

Input: grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
Output: [[2,2,2],[2,1,2],[2,2,2]]



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 50
1 <= grid[i][j], color <= 1000
0 <= row < m
0 <= col < n

*/

pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
    let row = row as usize;
    let col = col as usize;
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut grid = grid;
    let mut queue = std::collections::VecDeque::new();
    let mut is_visited = vec![vec![false; m]; n];
    let mut border_cells = vec![];

    let original_color = grid[row][col];
    queue.push_back((row, col));
    is_visited[row][col] = true;

    while let Some((i, j)) = queue.pop_front() {
        let mut is_border = false;
        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                is_border = true;
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if grid[ni][nj] != original_color {
                is_border = true;
            } else if !is_visited[ni][nj] {
                is_visited[ni][nj] = true;
                queue.push_back((ni, nj));
            }
        }

        if is_border {
            border_cells.push((i, j));
        }
    }

    for (i, j) in border_cells {
        grid[i][j] = color;
    }

    grid
}