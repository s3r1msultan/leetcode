/*Write a program to solve a Sudoku puzzle by filling the empty cells.

A sudoku solution must satisfy all of the following rules:

Each of the digits 1-9 must occur exactly once in each row.
Each of the digits 1-9 must occur exactly once in each column.
Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.

The '.' character indicates empty cells.



Example 1:

Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
Explanation: The input board is shown above and the only valid solution is shown below:




Constraints:

board.length == 9
board[i].length == 9
board[i][j] is a digit or '.'.
It is guaranteed that the input board has only one solution.

*/
// use std::collections::HashSet;
//
// pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
//     fn backtrack(board: &mut Vec<Vec<char>>, i: usize, j: usize, set_of_squares: &mut Vec<HashSet<char>>, set_of_columns: &mut Vec<HashSet<char>>, set_of_rows: &mut Vec<HashSet<char>>) -> bool {
//         if i == 9 {
//             return true;
//         }
//
//         let square = (i / 3) * 3 + j / 3;
//
//         let (next_i, next_j) = if j == 8 { (i + 1, 0) } else { (i, j + 1) };
//
//         if board[i][j] != '.' {
//             return backtrack(board, next_i, next_j, set_of_squares, set_of_columns, set_of_rows);
//         }
//
//         for digit in '1'..='9' {
//             if set_of_squares[square].contains(&digit) || set_of_columns[i].contains(&digit) || set_of_rows[j].contains(&digit) {
//                 continue;
//             }
//
//             set_of_squares[square].insert(digit);
//             set_of_columns[i].insert(digit);
//             set_of_rows[j].insert(digit);
//
//             board[i][j] = digit;
//
//             if backtrack(board, next_i, next_j, set_of_squares, set_of_columns, set_of_rows) {
//                 return true;
//             }
//
//             board[i][j] = '.';
//
//             set_of_squares[square].remove(&digit);
//             set_of_columns[i].remove(&digit);
//             set_of_rows[j].remove(&digit);
//         }
//
//         false
//     }
//
//     let mut set_of_squares = vec![HashSet::new(); 9];
//     let mut set_of_columns = vec![HashSet::new(); 9];
//     let mut set_of_rows = vec![HashSet::new(); 9];
//
//     for i in 0..9 {
//         for j in 0..9 {
//             if board[i][j] == '.' {
//                 continue;
//             }
//
//             let square = (i / 3) * 3 + j / 3;
//
//             set_of_squares[square].insert(board[i][j]);
//             set_of_rows[j].insert(board[i][j]);
//             set_of_columns[i].insert(board[i][j]);
//         }
//     }
//
//     backtrack(board, 0, 0, &mut set_of_squares, &mut set_of_columns, &mut set_of_rows);
// }

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut rows = [[false; 9]; 9];
    let mut cols = [[false; 9]; 9];
    let mut squares = [[false; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                continue;
            }

            let k = (board[i][j] as u8 - b'1') as usize;

            rows[i][k] = true;
            cols[j][k] = true;

            let square = i / 3 * 3 + j / 3;
            squares[square][k] = true;
        }
    }

    fn backtrack(
        i: usize,
        j: usize,
        board: &mut Vec<Vec<char>>,
        rows: &mut [[bool; 9]; 9],
        cols: &mut [[bool; 9]; 9],
        squares: &mut [[bool; 9]; 9],
    ) -> bool {
        if i == 9 {
            return true;
        }

        if j == 9 {
            return backtrack(i + 1, 0, board, rows, cols, squares);
        }

        if board[i][j] != '.' {
            return backtrack(i, j + 1, board, rows, cols, squares);
        }

        let square = i / 3 * 3 + j / 3;

        for k in 0..9 {
            if rows[i][k] || cols[j][k] || squares[square][k] {
                continue;
            }

            rows[i][k] = true;
            cols[j][k] = true;
            squares[square][k] = true;
            board[i][j] = (k as u8 + b'1') as char;

            if backtrack(i, j + 1, board, rows, cols, squares) {
                return true;
            }

            rows[i][k] = false;
            cols[j][k] = false;
            squares[square][k] = false;
            board[i][j] = '.';
        }

        false
    }

    backtrack(0, 0, board, &mut rows, &mut cols, &mut squares);
}
