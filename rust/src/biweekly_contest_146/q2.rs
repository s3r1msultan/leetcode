/*You are given a 2D integer array grid with size m x n. You are also given an integer k.

Your task is to calculate the number of paths you can take from the top-left cell (0, 0) to the bottom-right cell (m - 1, n - 1) satisfying the following constraints:

You can either move to the right or down. Formally, from the cell (i, j) you may move to the cell (i, j + 1) or to the cell (i + 1, j) if the target cell exists.
The XOR of all the numbers on the path must be equal to k.

Return the total number of such paths.

Since the answer can be very large, return the result modulo 109 + 7.



Example 1:

Input: grid = [[2, 1, 5], [7, 10, 0], [12, 6, 4]], k = 11

Output: 3

Explanation:

The 3 paths are:

(0, 0) → (1, 0) → (2, 0) → (2, 1) → (2, 2)
(0, 0) → (1, 0) → (1, 1) → (1, 2) → (2, 2)
(0, 0) → (0, 1) → (1, 1) → (2, 1) → (2, 2)

Example 2:

Input: grid = [[1, 3, 3, 3], [0, 3, 3, 2], [3, 0, 1, 1]], k = 2

Output: 5

Explanation:

The 5 paths are:

(0, 0) → (1, 0) → (2, 0) → (2, 1) → (2, 2) → (2, 3)
(0, 0) → (1, 0) → (1, 1) → (2, 1) → (2, 2) → (2, 3)
(0, 0) → (1, 0) → (1, 1) → (1, 2) → (1, 3) → (2, 3)
(0, 0) → (0, 1) → (1, 1) → (1, 2) → (2, 2) → (2, 3)
(0, 0) → (0, 1) → (0, 2) → (1, 2) → (2, 2) → (2, 3)

Example 3:

Input: grid = [[1, 1, 1, 2], [3, 0, 3, 2], [3, 0, 2, 2]], k = 10

Output: 0



Constraints:

1 <= m == grid.length <= 300
1 <= n == grid[r].length <= 300
0 <= grid[r][c] < 16
0 <= k < 16©leetcode*/
pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let m = grid.len();
    let n = grid[0].len();

    let mut dp = vec![vec![vec![0; 16]; n]; m];
    dp[0][0][grid[0][0] as usize] = 1;

    for i in 0..m {
        for j in 0..n {
            for k in 0..16 {
                if dp[i][j][k] > 0 {
                    if i + 1 < m {
                        let new_xor = k ^ grid[i + 1][j] as usize;
                        dp[i + 1][j][new_xor] = (dp[i + 1][j][new_xor] + dp[i][j][k]) % MOD;
                    }

                    if j + 1 < n {
                        let new_xor = k ^ grid[i][j + 1] as usize;
                        dp[i][j + 1][new_xor] = (dp[i][j + 1][new_xor] + dp[i][j][k]) % MOD;
                    }
                }
            }
        }
    }

    dp[m - 1][n - 1][k as usize]
}