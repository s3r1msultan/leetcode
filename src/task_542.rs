/*Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.

The distance between two adjacent cells is 1.



Example 1:

Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
Output: [[0,0,0],[0,1,0],[0,0,0]]

Example 2:

Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
Output: [[0,0,0],[0,1,0],[1,2,1]]



Constraints:

m == mat.length
n == mat[i].length
1 <= m, n <= 104
1 <= m * n <= 104
mat[i][j] is either 0 or 1.
There is at least one 0 in mat.

*/

fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let n = mat.len();
    let m = mat[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut result = vec![vec![i32::MAX; m]; n];

    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 0 {
                result[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }


    while let Some((i, j)) = queue.pop_front() {
        let val = mat[i][j];
        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if val + 1 < result[ni][nj] {
                result[ni][nj] = val + 1;
                queue.push_back((ni, nj));
            }
        }
    }

    result
}


