/*There is a dungeon with n x m rooms arranged as a grid.

You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds when you can start moving to that room. You start from the room (0, 0) at time t = 0 and can move to an adjacent room. Moving between adjacent rooms takes exactly one second.

Return the minimum time to reach the room (n - 1, m - 1).

Two rooms are adjacent if they share a common wall, either horizontally or vertically.



Example 1:

Input: moveTime = [[0,4],[4,4]]

Output: 6

Explanation:

The minimum time required is 6 seconds.

At time t == 4, move from room (0, 0) to room (1, 0) in one second.
At time t == 5, move from room (1, 0) to room (1, 1) in one second.

Example 2:

Input: moveTime = [[0,0,0],[0,0,0]]

Output: 3

Explanation:

The minimum time required is 3 seconds.

At time t == 0, move from room (0, 0) to room (1, 0) in one second.
At time t == 1, move from room (1, 0) to room (1, 1) in one second.
At time t == 2, move from room (1, 1) to room (1, 2) in one second.

Example 3:

Input: moveTime = [[0,1],[1,2]]

Output: 3



Constraints:

2 <= n == moveTime.length <= 50
2 <= m == moveTime[i].length <= 50
0 <= moveTime[i][j] <= 109

*/

pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let m = move_time.len();
    let n = move_time[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut heap = BinaryHeap::new();
    let mut minimum_time = vec![vec![i32::MAX; n]; m];

    heap.push(Reverse((0, 0, 0)));
    minimum_time[0][0] = 0;

    while let Some(Reverse((time, row, col))) = heap.pop() {
        if row == m - 1 && col == n - 1 {
            return time;
        }

        for &(dr, dc) in &directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row < 0 || new_col < 0 || new_row >= m as i32 || new_col >= n as i32 {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            let new_time = if time >= move_time[new_row][new_col] {
                time
            } else {
                move_time[new_row][new_col]
            };

            if new_time + 1 < minimum_time[new_row][new_col] {
                minimum_time[new_row][new_col] = new_time + 1;
                heap.push(Reverse((new_time + 1, new_row, new_col)));
            }
        }
    }

    unreachable!()
}