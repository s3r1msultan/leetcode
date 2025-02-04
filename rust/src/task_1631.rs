/*You are a hiker preparing for an upcoming hike. You are given heights, a 2D array of size rows x columns, where heights[row][col] represents the height of cell (row, col). You are situated in the top-left cell, (0, 0), and you hope to travel to the bottom-right cell, (rows-1, columns-1) (i.e., 0-indexed). You can move up, down, left, or right, and you wish to find a route that requires the minimum effort.

A route's effort is the maximum absolute difference in heights between two consecutive cells of the route.

Return the minimum effort required to travel from the top-left cell to the bottom-right cell.



Example 1:

Input: heights = [[1,2,2],[3,8,2],[5,3,5]]
Output: 2
Explanation: The route of [1,3,5,3,5] has a maximum absolute difference of 2 in consecutive cells.
This is better than the route of [1,2,2,2,5], where the maximum absolute difference is 3.

Example 2:

Input: heights = [[1,2,3],[3,8,4],[5,3,5]]
Output: 1
Explanation: The route of [1,2,3,4,5] has a maximum absolute difference of 1 in consecutive cells, which is better than route [1,3,5,3,5].

Example 3:

Input: heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
Output: 0
Explanation: This route does not require any effort.



Constraints:

rows == heights.length
columns == heights[i].length
1 <= rows, columns <= 100
1 <= heights[i][j] <= 106

*/


pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = heights.len();
    let m = heights[0].len();
    let directions = [(0,1), (1,0), (0,-1), (-1,0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    if n == 1 && m == 1 {
        return 0;
    }

    let mut min_heap = BinaryHeap::new();
    let mut diffs = vec![vec![i32::MAX; m]; n];
    diffs[0][0] = 0;
    min_heap.push((Reverse(0), 0, 0));


    while let Some(Reverse((curr_diff, i, j))) = min_heap.pop() {
        if i == n -1 && j == m -1 {
            return curr_diff;
        }

        let curr_height = heights[i][j];

        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }

            let ni = ni as usize;
            let nj = nj as usize;

            let next_diff = (curr_height - heights[ni][nj]).abs().max(curr_diff);

            if diffs[ni][nj] > next_diff {
                diffs[ni][nj] = next_diff;
                min_heap.push((Reverse(curr_diff), ni, nj));
            }
        }
    }

    0
}