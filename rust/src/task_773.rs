/*On an 2 x 3 board, there are five tiles labeled from 1 to 5, and an empty square represented by 0. A move consists of choosing 0 and a 4-directionally adjacent number and swapping it.

The state of the board is solved if and only if the board is [[1,2,3],[4,5,0]].

Given the puzzle board board, return the least number of moves required so that the state of the board is solved. If it is impossible for the state of the board to be solved, return -1.



Example 1:

Input: board = [[1,2,3],[4,0,5]]
Output: 1
Explanation: Swap the 0 and the 5 in one move.

Example 2:

Input: board = [[1,2,3],[5,4,0]]
Output: -1
Explanation: No number of moves will make the board solved.

Example 3:

Input: board = [[4,1,2],[5,0,3]]
Output: 5
Explanation: 5 is the smallest number of moves that solves the board.
An example path:
After move 0: [[4,1,2],[5,0,3]]
After move 1: [[4,1,2],[0,5,3]]
After move 2: [[0,1,2],[4,5,3]]
After move 3: [[1,0,2],[4,5,3]]
After move 4: [[1,2,0],[4,5,3]]
After move 5: [[1,2,3],[4,5,0]]



Constraints:

board.length == 2
board[i].length == 3
0 <= board[i][j] <= 5
Each value board[i][j] is unique.

*/

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    let n = 2;
    let m = 3;
    let mut set = std::collections::HashSet::new();
    let right_board = vec![vec![1, 2, 3], vec![4, 5, 0]];
    let mut queue = std::collections::VecDeque::new();

    for i in 0..2 {
        for j in 0..3 {
            if board[i][j] == 0 {
                queue.push_back((board.clone(), (i, j), 0));
                break;
            }
        }
    }

    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut min_count = i32::MAX;


    while !queue.is_empty() {
        let k = queue.len();
        for _ in 0..k {
            let (board, (y, x), count) = queue.pop_front().unwrap();

            if set.contains(&board) {
                continue;
            }

            if board == right_board {
                min_count = min_count.min(count);
            }

            for &(dx, dy) in &directions {
                let ny = y as i32 + dy;
                let nx = x as i32 + dx;
                if ny < 0 || nx < 0 || ny >= n || nx >= m {
                    continue;
                }
                let ny = ny as usize;
                let nx = nx as usize;

                let mut new_board = board.clone();

                let temp = new_board[y][x];
                new_board[y][x] = new_board[ny][nx];
                new_board[ny][nx] = temp;

                queue.push_back((new_board, (ny, nx), count + 1));
            }

            set.insert(board);
        }
    }

    if min_count == i32::MAX {
        -1
    } else {
        min_count
    }
}