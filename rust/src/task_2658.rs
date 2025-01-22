/*You are given a 0-indexed 2D matrix grid of size m x n, where (r, c) represents:

A land cell if grid[r][c] = 0, or
A water cell containing grid[r][c] fish, if grid[r][c] > 0.

A fisher can start at any water cell (r, c) and can do the following operations any number of times:

Catch all the fish at cell (r, c), or
Move to any adjacent water cell.

Return the maximum number of fish the fisher can catch if he chooses his starting cell optimally, or 0 if no water cell exists.

An adjacent cell of the cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) or (r - 1, c) if it exists.



Example 1:

Input: grid = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]]
Output: 7
Explanation: The fisher can start at cell (1,3) and collect 3 fish, then move to cell (2,3) and collect 4 fish.

Example 2:

Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,1]]
Output: 1
Explanation: The fisher can start at cells (0,0) or (3,3) and collect a single fish.



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 10
0 <= grid[i][j] <= 10

*/
use std::cmp::Ordering;

struct DS {
    representatives: Vec<usize>,
    fishes: Vec<i32>,
    ranks: Vec<i32>,
}

impl DS {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            fishes: vec![0; n],
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
                self.fishes[root_a] += self.fishes[root_b];
            }
            Ordering::Equal => {
                self.representatives[root_b] = root_a;
                self.ranks[root_a] += 1;
                self.fishes[root_a] += self.fishes[root_b];
            }
            Ordering::Greater => {
                self.representatives[root_b] = root_a;
                self.fishes[root_b] += self.fishes[root_a];
            }
        }
    }
}

fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
    i >= 0 && j >= 0 && i < n && j < m
}

pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(-1, 0), (0, -1)];
    let mut set = DS::new(n * m);

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 {
                continue;
            }
            let curr_index = i * m + j;
            if set.fishes[curr_index] == 0 {
                set.fishes[curr_index] = grid[i][j];
            }
            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if !is_valid(ni, nj, n as i32, m as i32) {
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if grid[ni][nj] == 0 {
                    continue;
                }

                let next_index = ni * m + nj;
                if set.fishes[next_index] == 0 {
                    set.fishes[next_index] = grid[ni][nj];
                }
                set.union(curr_index, next_index);
            }
        }
    }

    let mut max = 0;
    for i in 0..n * m {
        max = max.max(set.fishes[i]);
    }
    max
}