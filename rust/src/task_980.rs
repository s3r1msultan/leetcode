/*You are given an m x n integer array grid where grid[i][j] could be:

1 representing the starting square. There is exactly one starting square.
2 representing the ending square. There is exactly one ending square.
0 representing empty squares we can walk over.
-1 representing obstacles that we cannot walk over.

Return the number of 4-directional walks from the starting square to the ending square, that walk over every non-obstacle square exactly once.



Example 1:

Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
Output: 2
Explanation: We have the following two paths:
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)

Example 2:

Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,2]]
Output: 4
Explanation: We have the following four paths:
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)

Example 3:

Input: grid = [[0,1],[2,0]]
Output: 0
Explanation: There is no path that walks over every empty square exactly once.
Note that the starting and ending square can be anywhere in the grid.



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 20
1 <= m * n <= 20
-1 <= grid[i][j] <= 2
There is exactly one starting cell and one ending cell.

*/

pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let mut start = (0, 0);
    let mut free_cells = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                0 => free_cells += 1,
                1 => start = (i, j),
                _ => unreachable!()
            }
        }
    }

    fn backtrack(grid: &mut Vec<Vec<i32>>, result: &mut i32, free_cells: i32, steps: i32, i: usize, j: usize) {
        if grid[i][j] == 2 {
            if steps - 1 == free_cells {
                *result += 1;
            }
            return;
        }
        grid[i][j] = 1;
        let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (dx, dy) in directions {
            let nx = i as isize + dx;
            let ny = j as isize + dy;

            if nx >= 0 && ny >= 0 && nx < grid.len() as isize && ny < grid[0].len() as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[nx][ny] == 0 || grid[nx][ny] == 2 {
                    backtrack(grid, result, free_cells, steps + 1, nx, ny);
                }
            }
        }
        grid[i][j] = 0;
    }
    let mut result = 0;
    backtrack(&grid, &mut result, free_cells, 0, start.0, start.1);
    result
}