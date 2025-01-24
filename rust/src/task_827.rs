/*You are given an n x n binary matrix grid. You are allowed to change at most one 0 to be 1.

Return the size of the largest island in grid after applying this operation.

An island is a 4-directionally connected group of 1s.



Example 1:

Input: grid = [[1,0],[0,1]]
Output: 3
Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.

Example 2:

Input: grid = [[1,1],[1,0]]
Output: 4
Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.

Example 3:

Input: grid = [[1,1],[1,1]]
Output: 4
Explanation: Can't change any 0 to 1, only one island with area = 4.



Constraints:

n == grid.length
n == grid[i].length
1 <= n <= 500
grid[i][j] is either 0 or 1.

*/
struct DS {
    representatives: Vec<usize>,
    size: Vec<i32>,
}

impl DS {
    fn new(n: usize) -> Self {
        Self {
            representatives: (0..n).collect(),
            size: vec![1; n],
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

        if self.size[root_a] > self.size[root_b] {
            self.representatives[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
            self.size[root_b] = 0;
        } else {
            self.representatives[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
            self.size[root_a] = 0;
        }
    }

    fn get_max_size(&self) -> i32 {
        self.size.iter().fold(0, |acc, &curr| acc + if curr > 1 { curr } else { 0 })
    }

    fn get_size_of(&self, x: usize) -> i32 {
        self.size[x]
    }
}
pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut disjoint_set = DS::new(n * m);
    let mut zero_positions = vec![];
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                let index = i * m + j;

                for &(di, dj) in &directions {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if !is_valid(ni, nj, n as i32, m as i32) || grid[ni][nj] == 0 {
                        continue;
                    }

                    let ni = ni as usize;
                    let nj = nj as usize;

                    let neighbour_index = ni * m + nj;

                    disjoint_set.union(index, neighbour_index);
                }
            } else {
                zero_positions.push((i, j));
            }
        }
    }


    let mut max = disjoint_set.get_max_size();

    for (i, j) in zero_positions {
        let mut sum = 0;
        let mut hashset = std::collections::HashSet::new();
        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) || grid[ni][nj] == 0 {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            let neighbour_index = ni * m + nj;
            let root = disjoint_set.find(neighbour_index);

            if hashset.contains(&root) {
                continue;
            }

            sum += disjoint_set.get_size_of(root);

            hashset.insert(root);
        }

        max = max.max(sum);
    }

    max
}