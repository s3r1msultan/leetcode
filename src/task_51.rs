/*The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.

Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.



Example 1:

Input: n = 4
Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above

Example 2:

Input: n = 1
Output: [["Q"]]



Constraints:

1 <= n <= 9

*/

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    use std::collections::HashSet;
    fn backtrack(curr: &mut Vec<Vec<char>>, columns: &mut Vec<bool>, diagonals_1: &mut Vec<bool>, diagonals_2: &mut Vec<bool>, i: usize, n: usize, result: &mut HashSet<Vec<String>>) {
        if i == n {
            result.insert(curr.clone().into_iter().map(|v| v.into_iter().collect::<String>()).collect());
            return;
        }

        for j in 0..n {
            let d1 = i + j;
            let d2 = i + (n - j - 1);

            if columns[j] || diagonals_1[d1] || diagonals_2[d2] {
                continue;
            }

            columns[j] = true;
            diagonals_1[d1] = true;
            diagonals_2[d2] = true;

            curr[i][j] = 'Q';
            backtrack(curr, columns, diagonals_1, diagonals_2, i + 1, n, result);
            curr[i][j] = '.';

            columns[j] = false;
            diagonals_1[d1] = false;
            diagonals_2[d2] = false;
        }
    }

    let n = n as usize;
    let mut result = HashSet::new();
    let mut curr = vec![vec!['.'; n]; n];
    let mut columns = vec![false; n];
    let mut diagonals_1 = vec![false; 2*n];
    let mut diagonals_2 = vec![false; 2*n];

    backtrack(&mut curr, &mut columns, &mut diagonals_1, &mut diagonals_2, 0, n, &mut result);
    result.into_iter().collect()
}
