/*A peak element in a 2D grid is an element that is strictly greater than all of its adjacent neighbors to the left, right, top, and bottom.

Given a 0-indexed m x n matrix mat where no two adjacent cells are equal, find any peak element mat[i][j] and return the length 2 array [i,j].

You may assume that the entire matrix is surrounded by an outer perimeter with the value -1 in each cell.

You must write an algorithm that runs in O(m log(n)) or O(n log(m)) time.



Example 1:

Input: mat = [[1,4],[3,2]]
Output: [0,1]
Explanation: Both 3 and 4 are peak elements so [1,0] and [0,1] are both acceptable answers.

Example 2:

Input: mat = [[10,20,15],[21,30,14],[7,16,32]]
Output: [1,1]
Explanation: Both 30 and 32 are peak elements so [1,1] and [2,2] are both acceptable answers.



Constraints:

m == mat.length
n == mat[i].length
1 <= m, n <= 500
1 <= mat[i][j] <= 105
No two adjacent cells are equal.

*/

pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let n = mat.len();
    let m = mat[0].len();
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    fn is_valid(i: i32, j: i32, n: i32, m: i32) -> bool {
        i >= 0 && j >= 0 && i < n && j < m
    }

    for i in 0..n {
        let mut start = 0;
        let mut end = m;

        while start < end {
            let mid = start + (end - start) / 2;

            if mid > 0 && mat[i][mid] > mat[i][mid - 1] && mid < m - 1 && mat[i][mid] > mat[i][mid + 1] {
                end = mid;
                break;
            }

            if mid > 0 && mat[i][mid] > mat[i][mid - 1] {
                start = mid + 1;
            } else {
                end = mid;
            }
        }

        for &(di, dj) in &directions {
            let ni = i as i32 + di;
            let nj = end as i32 + dj;
            
            if !is_valid(ni, nj, n as i32, m as i32) {
                continue;
            }
            
            let ni = ni as usize;
            let nj  = nj as usize;
            
            if mat[i][end] <= 
        }
    }


    vec![-1, -1]
}