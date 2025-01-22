/*Given a 2D grid consists of 0s (land) and 1s (water).  An island is a maximal 4-directionally connected group of 0s and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.

Return the number of closed islands.



Example 1:

Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
Output: 2
Explanation:
Islands in gray are closed because they are completely surrounded by water (group of 1s).

Example 2:

Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
Output: 1

Example 3:

Input: grid = [[1,1,1,1,1,1,1],
[1,0,0,0,0,0,1],
[1,0,1,1,1,0,1],
[1,0,1,0,1,0,1],
[1,0,1,1,1,0,1],
[1,0,0,0,0,0,1],
[1,1,1,1,1,1,1]]
Output: 2



Constraints:

1 <= grid.length, grid[0].length <= 100
0 <= grid[i][j] <=1

*/

pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    struct UnionFind {
        representatives: Vec<usize>,
        pub boundaries: Vec<bool>,
    }
    impl UnionFind {
        fn new(n: usize, m: usize) -> Self {
            Self {
                representatives: (0..n * m).collect(),
                boundaries: vec![false; n * m],
            }
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

        fn find(&mut self, x: usize) -> usize {
            let parent = self.representatives[x];
            if parent == x {
                return x;
            }

            let root = self.find(parent);
            self.representatives[x] = root;

            root
        }
    }
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }
    fn is_boundary(i: usize, j: usize, n: usize, m: usize) -> bool {
        i == 0 || j == 0 || i == n - 1 || j == m - 1
    }
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(-1, 0), (0, -1)];
    let mut union_find = UnionFind::new(n, m);
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                continue;
            }

            let curr_index = i * m + j;
            if is_boundary(i, j, n, m) {
                union_find.boundaries[curr_index] = true;
            }

            for &(dx, dy) in &directions {
                let nx = j as i32 + dx;
                let ny = i as i32 + dy;

                if !is_valid(ny, nx, n as i32, m as i32) {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if grid[ny][nx] == 1 {
                    continue;
                }

                let next_index = ny * m + nx;
                union_find.union(curr_index, next_index);
            }
        }
    }

    println!("{:?}", union_find.representatives);
    for chunk in union_find.representatives.chunks(n) {
        println!("{:?}", chunk);
    }
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                continue;
            }
            let curr_index = i * m + j;
            let representative = union_find.find(curr_index);
            if !union_find.boundaries[representative] {
                set.insert(representative);
            }
        }
    }

    set.len() as i32
}