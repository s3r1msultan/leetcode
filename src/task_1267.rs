/*You are given a map of a server center, represented as a m * n integer matrix grid, where 1 means that on that cell there is a server and 0 means that it is no server. Two servers are said to communicate if they are on the same row or on the same column.

Return the number of servers that communicate with any other server.



Example 1:

Input: grid = [[1,0],[0,1]]
Output: 0
Explanation: No servers can communicate with others.

Example 2:

Input: grid = [[1,0],[1,1]]
Output: 3
Explanation: All three servers can communicate with at least one other server.

Example 3:

Input: grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
Output: 4
Explanation: The two servers in the first row can communicate with each other. The two servers in the third column can communicate with each other. The server at right bottom corner can't communicate with any other server.



Constraints:

m == grid.length
n == grid[i].length
1 <= m <= 250
1 <= n <= 250
grid[i][j] == 0 or 1

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
        if root_b == root_a {
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

    fn get_connected_servers(&self) -> i32 {
        self.size.iter().fold(0, |acc, &curr| acc + if curr > 1 { curr } else { 0 })
    }
}

pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut set = DS::new(n * m);

    let mut row_map = vec![vec![]; n];
    let mut col_map = vec![vec![]; m];

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 {
                let index = i * m + j;
                row_map[i].push(index);
                col_map[j].push(index);
            }
        }
    }

    for row in row_map {
        for i in 1..row.len() {
            set.union(row[i - 1], row[i]);
        }
    }

    for col in col_map {
        for j in 1..col.len() {
            set.union(col[j - 1], col[j]);
        }
    }

    set.get_connected_servers()
}