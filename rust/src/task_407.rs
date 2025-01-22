/*Given an m x n integer matrix heightMap representing the height of each unit cell in a 2D elevation map, return the volume of water it can trap after raining.



Example 1:

Input: heightMap = [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]
Output: 4
Explanation: After the rain, water is trapped between the blocks.
We have two small ponds 1 and 3 units trapped.
The total volume of water trapped is 4.

Example 2:

Input: heightMap = [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]
Output: 10



Constraints:

m == heightMap.length
n == heightMap[i].length
1 <= m, n <= 200
0 <= heightMap[i][j] <= 2 * 104

*/

pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = height_map.len();
    let m = height_map[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut heap = BinaryHeap::new();
    let mut is_visited = vec![vec![false; m]; n];


    for i in 0..n {
        is_visited[i][0] = true;
        is_visited[i][m - 1] = true;

        heap.push(Reverse((height_map[i][0], i, 0)));
        heap.push(Reverse((height_map[i][m - 1], i, m - 1)));
    }

    for j in 0..m {
        is_visited[0][j] = true;
        is_visited[n - 1][j] = true;

        heap.push(Reverse((height_map[0][j], 0, j)));
        heap.push(Reverse((height_map[n - 1][j], n - 1, j)));
    }

    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    let mut water = 0;

    while let Some(Reverse((height, i, j))) = heap.pop() {
        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            if !is_visited[ni][nj] {
                is_visited[ni][nj] = true;
                water += 0.max(height - height_map[ni][nj]);
                heap.push(Reverse((height_map[ni][nj].max(height), ni, nj)));
            }
        }
    }

    water
}