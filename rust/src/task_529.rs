/*Let's play the minesweeper game (Wikipedia, online game)!

You are given an m x n char matrix board representing the game board where:

'M' represents an unrevealed mine,
'E' represents an unrevealed empty square,
'B' represents a revealed blank square that has no adjacent mines (i.e., above, below, left, right, and all 4 diagonals),
digit ('1' to '8') represents how many mines are adjacent to this revealed square, and
'X' represents a revealed mine.

You are also given an integer array click where click = [clickr, clickc] represents the next click position among all the unrevealed squares ('M' or 'E').

Return the board after revealing this position according to the following rules:

If a mine 'M' is revealed, then the game is over. You should change it to 'X'.
If an empty square 'E' with no adjacent mines is revealed, then change it to a revealed blank 'B' and all of its adjacent unrevealed squares should be revealed recursively.
If an empty square 'E' with at least one adjacent mine is revealed, then change it to a digit ('1' to '8') representing the number of adjacent mines.
Return the board when no more squares will be revealed.



Example 1:

Input: board = [["E","E","E","E","E"],["E","E","M","E","E"],["E","E","E","E","E"],["E","E","E","E","E"]], click = [3,0]
Output: [["B","1","E","1","B"],["B","1","M","1","B"],["B","1","1","1","B"],["B","B","B","B","B"]]

Example 2:

Input: board = [["B","1","E","1","B"],["B","1","M","1","B"],["B","1","1","1","B"],["B","B","B","B","B"]], click = [1,2]
Output: [["B","1","E","1","B"],["B","1","X","1","B"],["B","1","1","1","B"],["B","B","B","B","B"]]



Constraints:

m == board.length
n == board[i].length
1 <= m, n <= 50
board[i][j] is either 'M', 'E', 'B', or a digit from '1' to '8'.
click.length == 2
0 <= clickr < m
0 <= clickc < n
board[clickr][clickc] is either 'M' or 'E'.

*/

pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
    let mut board = board;
    let (i, j) = (click[0] as usize, click[1] as usize);

    if board[i][j] == 'M' {
        board[i][j] = 'X';
        return board;
    }

    let n = board.len();
    let m = board[0].len();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((i, j));

    while let Some((i, j)) = queue.pop_front() {
        if board[i][j] == 'E' {
            let mut mines = 0;
            let mut neighbours = vec![];
            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if !is_valid(ni, nj, n as i32, m as i32) {
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if board[ni][nj] == 'M' {
                    mines += 1;
                } else if board[ni][nj] == 'E' {
                    neighbours.push((ni, nj));
                }
            }

            if mines > 0 {
                board[i][j] = (mines + '0' as u8) as char;
            } else {
                board[i][j] = 'B';
                for neighbour in neighbours {
                    queue.push_back(neighbour);
                }
            }
        }
    }

    board
}