/*You are given an m x n matrix board containing letters 'X' and 'O', capture regions that are surrounded:

Connect: A cell is connected to adjacent cells horizontally or vertically.
Region: To form a region connect every 'O' cell.
Surround: The region is surrounded with 'X' cells if you can connect the region with 'X' cells and none of the region cells are on the edge of the board.

To capture a surrounded region, replace all 'O's with 'X's in-place within the original board. You do not need to return anything.



Example 1:

Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]

Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]

Explanation:

In the above diagram, the bottom region is not captured because it is on the edge of the board and cannot be surrounded.

Example 2:

Input: board = [["X"]]

Output: [["X"]]



Constraints:

m == board.length
n == board[i].length
1 <= m, n <= 200
board[i][j] is 'X' or 'O'.

*/
use std::cmp::Ordering;
use std::collections::VecDeque;

struct UnionFind {
    representatives: Vec<usize>,
    ranks: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            ranks: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.representatives[x];
        if parent == x {
            return x;
        }
        let root = self.find(parent);
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

pub fn solve(board: &mut Vec<Vec<char>>) {
    let n = board.len();
    let m = board[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }
    fn is_boundary(i: usize, j: usize, n: usize, m: usize) -> bool {
        i == 0 || j == 0 || i == n - 1 || j == m - 1
    }

    fn bfs(board: &Vec<Vec<char>>, union_find: &mut UnionFind, i: usize, j: usize, n: usize, m: usize, directions: &[(i32, i32); 4], is_visited: &mut Vec<Vec<bool>>) {
        let mut queue = VecDeque::new();
        queue.push_back((i, j));

        while let Some((i, j)) = queue.pop_front() {
            let curr_index = i * m + j;

            for &(di, dj) in directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if !is_valid(ni, nj, i as i32, j as i32) {
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if board[ni][nj] == 'X' {
                    continue;
                }

                if !is_visited[ni][nj] {
                    is_visited[ni][nj] = true;
                    let next_index = ni * m + nj;
                    union_find.union(curr_index, next_index);
                    queue.push_back((ni, nj));
                }
            }
        }
    }

    let mut union_find = UnionFind::new(n * m);
    let mut is_visited = vec![vec![false; m]; n];

    for i in [0, n - 1] {
        for j in 0..m {
            if board[i][j] == 'X' {
                continue;
            }
            is_visited[i][j] = true;

            bfs(&board, &mut union_find, i, j, n, m, &directions, &mut is_visited);
        }
    }

    for i in 0..n {
        for j in [0, m - 1] {
            if board[i][j] == 'X' {
                continue;
            }
            is_visited[i][j] = true;

            bfs(&board, &mut union_find, i, j, n, m, &directions, &mut is_visited);
        }
    }


    for i in 0..n {
        for j in 0..m {
            let index = i * m + j;

            let root = union_find.find(index);

            let root_i = root / m;
            let root_j = root % m;

            if !is_boundary(root_i, root_j, n, m) {
                board[i][j] = 'X';
            }
        }
    }
}