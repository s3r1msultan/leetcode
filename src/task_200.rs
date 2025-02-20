// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.
//
// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
//
//
//
// Example 1:
//
// Input: grid = [
// ["1","1","1","1","0"],
// ["1","1","0","1","0"],
// ["1","1","0","0","0"],
// ["0","0","0","0","0"]
// ]
// Output: 1
//
// Example 2:
//
// Input: grid = [
// ["1","1","0","0","0"],
// ["1","1","0","0","0"],
// ["0","0","1","0","0"],
// ["0","0","0","1","1"]
// ]
// Output: 3
//
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] is '0' or '1'.
//


pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let mut grid = grid;

    fn dfs(grid: &mut Vec<Vec<char>>, i:usize, j:usize, max_rows: usize, max_columns: usize) {
        if i >= max_rows || j >= max_columns || grid[i][j] == '0' {
            return;
        }

        grid[i][j] = '0';

        if i > 0 { dfs(grid, i - 1 , j, max_rows, max_columns)}
        if j > 0 { dfs(grid, i , j - 1 , max_rows, max_columns)}
        if i + 1 < max_rows {dfs(grid, i+1, j, max_rows, max_columns)}
        if j + 1 < max_columns {dfs(grid, i, j+1, max_rows, max_columns)}

    }

    let mut res = 0;
    let max_rows = grid.len();
    let max_columns = grid[0].len();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' {
                res += 1;
                dfs(&mut grid, i, j, max_rows, max_columns);
            }
        }
    }


    return res;
}

#[cfg(test)]

#[test]

fn test_num_islands() {
    let grid = vec![
        vec!['1','1','1','1','0'],
        vec!['1','1','0','1','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','0','0','0']
    ];

    assert_eq!(num_islands(grid), 1);



    let grid = vec![
        vec!['1','1','0','0','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','1','0','0'],
        vec!['0','0','0','1','1']
    ];

    assert_eq!(num_islands(grid), 3);
}