use std::{usize, vec};

// Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.
//
//
//
// Example 1:
//
// Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
// Output: [1,2,4,7,5,3,6,8,9]
//
// Example 2:
//
// Input: mat = [[1,2],[3,4]]
// Output: [1,2,3,4]
//
//
//
// Constraints:
//
//     m == mat.length
//     n == mat[i].length
//     1 <= m, n <= 104
//     1 <= m * n <= 104
//     -105 <= mat[i][j] <= 105
//
//
pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let n = mat.len();
    let m = mat[0].len();
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }
    let mut result = vec![];

    for i in 0..n + m - 1 {
        let range: Vec<i32> = if i % 2 == 0 {
            (0..=i as i32).collect()
        } else {
            (0..=i as i32).rev().collect()
        };

        for j in range {
            let ni = j;
            let nj = i as i32 - j;
            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;

            result.push(mat[ni][nj]);
        }
    }

    result
}
