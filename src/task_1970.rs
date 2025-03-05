/*There is a 1-based binary matrix where 0 represents land and 1 represents water. You are given integers row and col representing the number of rows and columns in the matrix, respectively.

Initially on day 0, the entire matrix is land. However, each day a new cell becomes flooded with water. You are given a 1-based 2D array cells, where cells[i] = [ri, ci] represents that on the ith day, the cell on the rith row and cith column (1-based coordinates) will be covered with water (i.e., changed to 1).

You want to find the last day that it is possible to walk from the top to the bottom by only walking on land cells. You can start from any cell in the top row and end at any cell in the bottom row. You can only travel in the four cardinal directions (left, right, up, and down).

Return the last day where it is possible to walk from the top to the bottom by only walking on land cells.



Example 1:

Input: row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
Output: 2
Explanation: The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 2.

Example 2:

Input: row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
Output: 1
Explanation: The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 1.

Example 3:

Input: row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
Output: 3
Explanation: The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 3.



Constraints:

2 <= row, col <= 2 * 104
4 <= row * col <= 2 * 104
cells.length == row * col
1 <= ri <= row
1 <= ci <= col
All the values of cells are unique.

*/


struct DS {
    representatives: Vec<usize>,
    ranks: Vec<i32>
}

impl DS {
    fn new(n: usize, m: usize) -> Self {
        Self {
            representatives: (0..n*m).collect(),
            ranks: vec![1; n*m]
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

        if self.ranks[root_a] == self.ranks[root_b] {
            
        }
    }
}

pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let row = row as usize;
    let col = col as usize;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let cells = cells.into_iter().map(|v| v.into_iter().map(|x| x - 1).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut grid = vec![vec![0; col]; row];
    let mut floods = vec![0; row];

    for (k, cell) in cells.iter().enumerate() {
        let i = cell[0] as usize;
        let j = cell[1] as usize;
        grid[i][j] = 1;
        floods[i] += 1;

        if floods[i] == col {
            return k as i32;
        }

        let mut is_visited = vec![vec![false; col]; row];
        let mut queue = std::collections::VecDeque::new();
        for j in 0..col {
            if grid[0][j] == 0 {
                queue.push_back((0, j));
                is_visited[0][j] = true;
            }
        }

        let mut is_found = true;

        while let Some((i, j)) = queue.pop_front() {
            if i == row - 1 {
                is_found = false;
                break;
            }

            for &(di, dj) in &directions {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if !is_valid(ni, nj, row as i32, col as i32) {
                    continue;
                }

                let ni = ni as usize;
                let nj = nj as usize;

                if !is_visited[ni][nj] && grid[ni][nj] == 0 {
                    is_visited[ni][nj] = true;
                    queue.push_back((ni, nj));
                }
            }
        }

        if is_found {
            return k as i32;
        }
    }

    -1
}
