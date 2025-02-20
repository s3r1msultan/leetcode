/*Given a m x n matrix mat and an integer k, return a matrix answer where each answer[i][j] is the sum of all elements mat[r][c] for:

i - k <= r <= i + k,
j - k <= c <= j + k, and
(r, c) is a valid position in the matrix.



Example 1:

Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
Output: [[12,21,16],[27,45,33],[24,39,28]]

Example 2:

Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 2
Output: [[45,45,45],[45,45,45],[45,45,45]]



Constraints:

m == mat.length
n == mat[i].length
1 <= m, n, k <= 100
1 <= mat[i][j] <= 100

*/

/*pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {

    fn max_depth(node: Option<Rc>) -> i32 {

    }
}*/

pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let directions = [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];
    let mut prefix_sum = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            prefix_sum[i][j] = mat[i][j];
            for &(dx, dy) in &directions {
                let nx = j as i32 + dx;
                let ny = i as i32 + dy;
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                prefix_sum[i][j] += prefix_sum[ny][nx];
            }
        }
    }

    println!("{:?}", prefix_sum);

    let mut result = vec![];

    result
}