/*Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.

The distance between two adjacent cells is 1.



Example 1:

Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
Output: [[0,0,0],[0,1,0],[0,0,0]]

Example 2:

Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
Output: [[0,0,0],[0,1,0],[1,2,1]]



Constraints:

m == mat.length
n == mat[i].length
1 <= m, n <= 104
1 <= m * n <= 104
mat[i][j] is either 0 or 1.
There is at least one 0 in mat.

*/

fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
	use std::collections::VecDeque;
	let mut queue = VecDeque::new();
	let m = mat.len();
	let n = mat[0].len();
	let mut result = vec![vec![0; n]; m];

	for i in 0..m {
		for j in 0..n {
			if mat[i][j] == 0 {
				queue.push_back((i, j));
			} else {
				result[i][j] = i32::MAX;
			}
		}
	}

	let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

	while let Some((i, j)) = queue.pop_front() {
		for &(dx, dy) in &directions {
			let nx = i as isize + dx;
			let ny = j as isize + dy;

			if nx >= 0 && ny >= 0 && nx < m as isize && ny < n as isize {
				let nx = nx as usize;
				let ny = ny as usize;
				if result[nx][ny] > result[i][j] + 1 {
					result[nx][ny] = result[i][j] + 1;
					queue.push_back((nx, ny));
				}
			}
		}
	}

	result
}


