/*Given an m x n binary matrix mat, return the number of special positions in mat.

A position (i, j) is called special if mat[i][j] == 1 and all other elements in row i and column j are 0 (rows and columns are 0-indexed).



Example 1:

Input: mat = [[1,0,0],[0,0,1],[1,0,0]]
Output: 1
Explanation: (1, 2) is a special position because mat[1][2] == 1 and all other elements in row 1 and column 2 are 0.

Example 2:

Input: mat = [[1,0,0],[0,1,0],[0,0,1]]
Output: 3
Explanation: (0, 0), (1, 1) and (2, 2) are special positions.



Constraints:

m == mat.length
n == mat[i].length
1 <= m, n <= 100
mat[i][j] is either 0 or 1.

*/
pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    let m = mat[0].len();
    let mut rows = vec![0; n];
    let mut cols = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 1 {
                rows[i] += 1;
                cols[j] += 1;
            }
        }
    }

    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if mat[i][j] == 1 && rows[i] == 1 && cols[j] == 1 {
                count += 1;
            }
        }
    }
    count
}