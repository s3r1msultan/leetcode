/* You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.

An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.

Return the number of islands in grid2 that are considered sub-islands.

 

Example 1:

Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
Output: 3
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.

Example 2:

Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
Output: 2 
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.

 

Constraints:

    m == grid1.length == grid2.length
    n == grid1[i].length == grid2[i].length
    1 <= m, n <= 500
    grid1[i][j] and grid2[i][j] are either 0 or 1.

 */

fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut visited = vec![vec![false; grid2[0].len()]; grid2.len()];
    for i in 0..grid2.len() {
        for j in 0..grid2[0].len() {
            if grid2[i][j] == 1 && !visited[i][j] {
                let mut is_sub_island = true;
                let mut stack = vec![(i, j)];
                visited[i][j] = true;
                while !stack.is_empty() {
                    let (x, y) = stack.pop().unwrap();
                    if grid1[x][y] == 0 {
                        is_sub_island = false;
                    }
                    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && nx < grid2.len() as i32 && ny >= 0 && ny < grid2[0].len() as i32 && grid2[nx as usize][ny as usize] == 1 && !visited[nx as usize][ny as usize] {
                            visited[nx as usize][ny as usize] = true;
                            stack.push((nx as usize, ny as usize));
                        }
                    }
                }
                if is_sub_island {
                    count += 1;
                }
            }
        }
    }

    count
}