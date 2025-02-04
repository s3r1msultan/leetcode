/*Given an m x n grid. Each cell of the grid has a sign pointing to the next cell you should visit if you are currently in this cell. The sign of grid[i][j] can be:

1 which means go to the cell to the right. (i.e go from grid[i][j] to grid[i][j + 1])
2 which means go to the cell to the left. (i.e go from grid[i][j] to grid[i][j - 1])
3 which means go to the lower cell. (i.e go from grid[i][j] to grid[i + 1][j])
4 which means go to the upper cell. (i.e go from grid[i][j] to grid[i - 1][j])

Notice that there could be some signs on the cells of the grid that point outside the grid.

You will initially start at the upper left cell (0, 0). A valid path in the grid is a path that starts from the upper left cell (0, 0) and ends at the bottom-right cell (m - 1, n - 1) following the signs on the grid. The valid path does not have to be the shortest.

You can modify the sign on a cell with cost = 1. You can modify the sign on a cell one time only.

Return the minimum cost to make the grid have at least one valid path.



Example 1:

Input: grid = [[1,1,1,1],[2,2,2,2],[1,1,1,1],[2,2,2,2]]
Output: 3
Explanation: You will start at point (0, 0).
The path to (3, 3) is as follows. (0, 0) --> (0, 1) --> (0, 2) --> (0, 3) change the arrow to down with cost = 1 --> (1, 3) --> (1, 2) --> (1, 1) --> (1, 0) change the arrow to down with cost = 1 --> (2, 0) --> (2, 1) --> (2, 2) --> (2, 3) change the arrow to down with cost = 1 --> (3, 3)
The total cost = 3.

Example 2:

Input: grid = [[1,1,3],[3,2,2],[1,1,4]]
Output: 0
Explanation: You can follow the path from (0, 0) to (2, 2).

Example 3:

Input: grid = [[1,2],[4,3]]
Output: 1



Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 100
1 <= grid[i][j] <= 4

*/

pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = grid.len();
    let m = grid[0].len();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut costs = vec![vec![0; m]; n];

    let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    min_heap.push(Reverse((0, 0, 0)));

    while let Some(Reverse((cost, i, j))) = min_heap.pop() {
        println!("{} {}", i, j);
        if i == n - 1 && j == m - 1 {
            return cost;
        }
        let k = grid[i][j] as usize - 1;
        let (ki, kj) = directions[k];

        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;
            let new_cost = cost + if ki == di && kj == dj {
                0
            } else {
                1
            };
            if costs[ni][nj] < new_cost {
                costs[ni][nj] = new_cost;
                min_heap.push(Reverse((new_cost, ni ,nj)));
            }
        }

    }

    -1
}