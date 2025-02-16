/*You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.

The area of an island is the number of cells with a value 1 in the island.

Return the maximum area of an island in grid. If there is no island, return 0.



Example 1:

Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
Output: 6
Explanation: The answer is not 11, because the island must be connected 4-directionally.

Example 2:

Input: grid = [[0,0,0,0,0,0,0,0]]
Output: 0



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 50
grid[i][j] is either 0 or 1.

*/

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    use std::collections::VecDeque;

    fn count(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back((i, j));
        let mut current_count = 0;

        while let Some((x, y)) = queue.pop_front() {
            if grid[x][y] == 1 {
                current_count += 1;
                grid[x][y] = 0;
                if x > 0 && grid[x - 1][y] == 1 {
                    queue.push_back((x - 1, y));
                }

                if y > 0 && grid[x][y - 1] == 1 {
                    queue.push_back((x, y - 1));
                }

                if x < grid.len() - 1 && grid[x + 1][y] == 1 {
                    queue.push_back((x + 1, y));
                }

                if y < grid[0].len() - 1 && grid[x][y + 1] == 1 {
                    queue.push_back((x, y + 1));
                }
            }
        }
        current_count
    }
    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                result = result.max(count(&mut grid, i, j));
            }
        }
    }


    result
}