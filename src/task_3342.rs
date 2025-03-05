/*You are given a n x n 2D array grid containing distinct elements in the range [0, n2 - 1].

Implement the NeighborSum class:

NeighborSum(int [][]grid) initializes the object.
int adjacentSum(int value) returns the sum of elements which are adjacent neighbors of value, that is either to the top, left, right, or bottom of value in grid.
int diagonalSum(int value) returns the sum of elements which are diagonal neighbors of value, that is either to the top-left, top-right, bottom-left, or bottom-right of value in grid.



Example 1:

Input:

["NeighborSum", "adjacentSum", "adjacentSum", "diagonalSum", "diagonalSum"]

[[[[0, 1, 2], [3, 4, 5], [6, 7, 8]]], [1], [4], [4], [8]]

Output: [null, 6, 16, 16, 4]

Explanation:

The adjacent neighbors of 1 are 0, 2, and 4.
The adjacent neighbors of 4 are 1, 3, 5, and 7.
The diagonal neighbors of 4 are 0, 2, 6, and 8.
The diagonal neighbor of 8 is 4.

Example 2:

Input:

["NeighborSum", "adjacentSum", "diagonalSum"]

[[[[1, 2, 0, 3], [4, 7, 15, 6], [8, 9, 10, 11], [12, 13, 14, 5]]], [15], [9]]

Output: [null, 23, 45]

Explanation:

The adjacent neighbors of 15 are 0, 10, 7, and 6.
The diagonal neighbors of 9 are 4, 12, 14, and 15.



Constraints:

3 <= n == grid.length == grid[0].length <= 10
0 <= grid[i][j] <= n2 - 1
All grid[i][j] are distinct.
value in adjacentSum and diagonalSum will be in the range [0, n2 - 1].
At most 2 * n2 calls will be made to adjacentSum and diagonalSum.

*/
use std::collections::HashMap;

const ADJACENT_DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const DIAGONAL_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
    i >= 0 && j >= 0 && i < n && j < m
}
struct NeighborSum {
    grid: Vec<Vec<i32>>,
    map: HashMap<i32, (i32, i32)>,
    n: usize,
    m: usize
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn sum(&self, value: i32, directions: &[(i32, i32);4]) -> i32 {
        let mut sum = 0;
        let &(i, j) = self.map.get(&value).unwrap();
        for &(di, dj) in &directions {
            let ni = i + di;
            let nj = j + dj;

            if !is_valid(ni ,nj, self.n as i32, self.m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            sum += self.grid[ni][nj];
        }
        sum
    }

    fn new(grid: Vec<Vec<i32>>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        let mut map = HashMap::new();
        for i in 0..n {
            for j in 0..m {
                map.insert(grid[i][j], (i as i32, j as i32));
            }
        }
        Self {
            n, m,
            grid,
            map
        }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.sum(value, &ADJACENT_DIRECTIONS)
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
       self.sum(value, &DIAGONAL_DIRECTIONS)
    }
}