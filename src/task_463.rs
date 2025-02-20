// You are given row x col grid representing a map where grid[i][j] = 1 represents land and grid[i][j] = 0 represents water.
//
// Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).
//
// The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.
//
//
//
// Example 1:
//
// Input: grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
// Output: 16
// Explanation: The perimeter is the 16 yellow stripes in the image above.
//
// Example 2:
//
// Input: grid = [[1]]
// Output: 4
//
// Example 3:
//
// Input: grid = [[1,0]]
// Output: 4
//
//
//
// Constraints:
//
// row == grid.length
// col == grid[i].length
// 1 <= row, col <= 100
// grid[i][j] is 0 or 1.
// There is exactly one island in grid.

/*pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                res += 4;
                if i != 0 && grid[i-1][j] == grid[i][j] {
                    res -= 2;
                }
                if j != 0 && grid[i][j-1] == grid[i][j] {
                    res -= 2;
                }
            }
        }
    }

    res
}
*/

pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                for &(di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if !is_valid(ni, nj, n as i32, m as i32) || grid[ni][nj] == 0 {
                        count += 1;
                        continue;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
#[test]

fn test_island_perimeter() {
    let grid = vec![vec![0, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 0, 0]];
    assert_eq!(island_perimeter(grid), 16);

    let grid = vec![vec![1]];
    assert_eq!(island_perimeter(grid), 4);

    let grid = vec![vec![1, 0]];
    assert_eq!(island_perimeter(grid), 4);
}