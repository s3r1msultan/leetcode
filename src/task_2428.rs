/*You are given an m x n integer matrix grid.

We define an hourglass as a part of the matrix with the following form:

Return the maximum sum of the elements of an hourglass.

Note that an hourglass cannot be rotated and must be entirely contained within the matrix.



Example 1:

Input: grid = [[6,2,1,3],[4,2,1,5],[9,2,8,7],[4,1,2,9]]
Output: 30
Explanation: The cells shown above represent the hourglass with the maximum sum: 6 + 2 + 1 + 2 + 9 + 2 + 8 = 30.

Example 2:

Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
Output: 35
Explanation: There is only one hourglass in the matrix, with the sum: 1 + 2 + 3 + 5 + 7 + 8 + 9 = 35.



Constraints:

m == grid.length
n == grid[i].length
3 <= m, n <= 150
0 <= grid[i][j] <= 106

*/

pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut prefix_sum = vec![vec![0; m - 2]; n - 2];
    let mut max_sum = 0;

    for i in 0..n - 2 {
        for j in 0..3 {
            prefix_sum[i][0] += grid[i][j];
            prefix_sum[i][0] += grid[i + 2][j];
        }
        prefix_sum[i][0] += grid[i + 1][1];
        max_sum = max_sum.max(prefix_sum[i][0]);
    }

    for i in 0..n - 2 {
        for j in 3..=m - 1 {
            prefix_sum[i][j - 2] += prefix_sum[i][j - 3];

            prefix_sum[i][j - 2] += grid[i][j];
            prefix_sum[i][j - 2] += grid[i + 1][j - 1];
            prefix_sum[i][j - 2] += grid[i + 2][j];

            prefix_sum[i][j - 2] -= grid[i][j - 3];
            prefix_sum[i][j - 2] -= grid[i + 1][j - 2];
            prefix_sum[i][j - 2] -= grid[i + 2][j - 3];

            max_sum = max_sum.max(prefix_sum[i][j - 2]);
        }
    }

    max_sum
}