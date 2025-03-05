/*The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

Given an integer n, return the number of distinct solutions to the n-queens puzzle.



Example 1:

Input: n = 4
Output: 2
Explanation: There are two distinct solutions to the 4-queens puzzle as shown.

Example 2:

Input: n = 1
Output: 1



Constraints:

1 <= n <= 9

*/

pub fn total_n_queens(n: i32) -> i32 {
    fn backtrack(columns: &mut Vec<bool>, diagonals_1: &mut Vec<bool>, diagonals_2: &mut Vec<bool>, i: usize, n: usize, result: &mut i32) {
        if i == n {
            *result += 1;
            return;
        }

        for j in 0..n {
            let d1 = i + j;
            let d2 = i + (n-j-1);

            if columns[j] || diagonals_1[d1] || diagonals_2[d2] {
                continue;
            }

            columns[j] = true;
            diagonals_1[d1] = true;
            diagonals_2[d2] = true;
            backtrack(columns, diagonals_1, diagonals_2, i+1, n, result);
            columns[j] = false;
            diagonals_1[d1] = false;
            diagonals_2[d2] = false;
        }
    }
    let n = n as usize;
    let mut columns = vec![false; n];
    let mut diagonals_1 = vec![false; 2*n];
    let mut diagonals_2 = vec![false; 2*n];

    let mut result = 0;
    backtrack(&mut columns, &mut diagonals_1, &mut diagonals_2, 0, n, &mut result);
    result
}