/*Given an m x n matrix board where each cell is a battleship 'X' or empty '.', return the number of the battleships on board.

Battleships can only be placed horizontally or vertically on board. In other words, they can only be made of the shape 1 x k (1 row, k columns) or k x 1 (k rows, 1 column), where k can be of any size. At least one horizontal or vertical cell separates between two battleships (i.e., there are no adjacent battleships).



Example 1:

Input: board = [["X",".",".","X"],[".",".",".","X"],[".",".",".","X"]]
Output: 2

Example 2:

Input: board = [["."]]
Output: 0



Constraints:

m == board.length
n == board[i].length
1 <= m, n <= 200
board[i][j] is either '.' or 'X'.



Follow up: Could you do it in one-pass, using only O(1) extra memory and without modifying the values board?
*/
use std::cmp::Ordering;

struct DS {
    representatives: Vec<usize>,
    ranks: Vec<i32>
}

impl DS {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            ranks: vec![1; n]
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.representatives[x];
        if parent == x {
            return x;
        }

        let root = self.find(x);
        self.representatives[x] = root;
        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        match self.ranks[root_a].cmp(&self.ranks[root_b]) {
            Ordering::Less => {
                self.representatives[root_a] = root_b;
            }
            Ordering::Equal => {
                self.representatives[root_b] = root_a;
                self.ranks[root_a] += 1;
            }
            Ordering::Greater => {
                self.representatives[root_b] = root_a;
            }
        }
    }
}

pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let n = board.len();
    let m = board[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut set = DS::new(n*m);
    for i in 0..n {
        for j in 0..m {
            if board[i][j] == '.' {
                continue;
            }

            let index = i * m + j;

            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if !is_valid(ni, nj, n as i32, m as i32) {
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if board[ni][nj] == '.' {
                    continue;
                }

                let nindex = ni * m + nj;

                set.union(index, nindex);
            }
        }
    }

    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            let index = i * m + j;
            if set.find(index) == index {
                count += 1;
            }
        }
    }

    count
}
