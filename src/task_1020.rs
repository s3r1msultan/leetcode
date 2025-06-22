/*You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.

A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.

Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.



Example 1:

Input: grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
Output: 3
Explanation: There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.

Example 2:

Input: grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
Output: 0
Explanation: All 1s are either on the boundary or can reach the boundary.



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 500
grid[i][j] is either 0 or 1.

*/

struct UnionFind {
    representatives: Vec<usize>,
    boundaries: Vec<bool>,
}

impl UnionFind {
    fn new(n: usize, m: usize) -> Self {
        Self {
            representatives: (0..n * m).collect(),
            boundaries: vec![false; n * m],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.representatives[x];
        if parent == x {
            return x;
        }

        let root = self.find(parent);
        self.representatives[parent] = root;

        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        if self.boundaries[root_a] {
            self.representatives[root_b] = root_a;
            self.boundaries[root_b] = true;
        } else if self.boundaries[root_b] {
            self.representatives[root_a] = root_b;
            self.boundaries[root_a] = true;
        } else {
            self.representatives[root_b] = root_a;
        }
    }
}

pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }
    fn is_boundary(i: usize, j: usize, n: usize, m: usize) -> bool {
        i == 0 || j == 0 || i == n - 1 || j == m - 1
    }

    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, -1), (-1, 0)];

    let mut union_find = UnionFind::new(n, m);

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 {
                continue;
            }
            let curr_index = i * n + j;
            if is_boundary(i, j, n, m) {
                union_find.boundaries[curr_index] = true;
            }

            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                
                if !is_valid(nj, ni, n as i32, m as i32) {
                    continue
                }
                
                let ni = ni as usize;
                let nj = nj as usize;
                
                if grid[ni][nj] == 1 {
                    let new_index = ni * n + nj;
                    if is_boundary(ni, nj, n, m) {
                        union_find.boundaries[new_index] = true;
                    }
                    union_find.union(curr_index, new_index);
                }
            }
        }
    }

    let mut enclaves = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                let parent = union_find.find(i * n + j);
                if !union_find.boundaries[parent] {
                    enclaves += 1
                }
            }
        }
    }

    enclaves
}