/*You are given two integers m and n representing a 0-indexed m x n grid. You are also given two 2D integer arrays guards and walls where guards[i] = [rowi, coli] and walls[j] = [rowj, colj] represent the positions of the ith guard and jth wall respectively.

A guard can see every cell in the four cardinal directions (north, east, south, or west) starting from their position unless obstructed by a wall or another guard. A cell is guarded if there is at least one guard that can see it.

Return the number of unoccupied cells that are not guarded.



Example 1:

Input: m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
Output: 7
Explanation: The guarded and unguarded cells are shown in red and green respectively in the above diagram.
There are a total of 7 unguarded cells, so we return 7.

Example 2:

Input: m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
Output: 4
Explanation: The unguarded cells are shown in green in the above diagram.
There are a total of 4 unguarded cells, so we return 4.



Constraints:

1 <= m, n <= 105
2 <= m * n <= 105
1 <= guards.length, walls.length <= 5 * 104
2 <= guards.length + walls.length <= m * n
guards[i].length == walls[j].length == 2
0 <= rowi, rowj < m
0 <= coli, colj < n
All the positions in guards and walls are unique.

*/

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let m = m as usize;
    let max = n.max(m);
    let mut grid = vec![vec![0; n]; m];

    for &wall in &walls {
        let i = wall[0];
        let j = wall[1];
        grid[i][j] = 2;
    }

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    for &guard in &guards {
        let i = guard[0];
        let j = guard[1];
        grid[i][j] = 1;
        for &direction in &directions {
            let di = direction.0 as isize;
            let dj = direction.1 as isize;
            for k in 1..max as isize {
                let dy = ((i as isize + di * k) as usize).min(m - 1);
                if grid[dy][j] == 2 {
                    break;
                } else {
                    grid[dy][j] = 1;
                }
                let dx = ((j as isize + dj * k) as usize).min(n - 1);
                if grid[i][dx] == 2 {
                    break;
                } else {
                    grid[dx][j] = 1;
                }
            }
        }
    }


    grid.into_iter().map(|vec| vec.iter().filter(|&&x| x == 1).sum()).sum()
}