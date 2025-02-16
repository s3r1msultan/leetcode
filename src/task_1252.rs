/*There is an m x n matrix that is initialized to all 0's. There is also a 2D array indices where each indices[i] = [ri, ci] represents a 0-indexed location to perform some increment operations on the matrix.

For each location indices[i], do both of the following:

Increment all the cells on row ri.
Increment all the cells on column ci.

Given m, n, and indices, return the number of odd-valued cells in the matrix after applying the increment to all locations in indices.



Example 1:

Input: m = 2, n = 3, indices = [[0,1],[1,1]]
Output: 6
Explanation: Initial matrix = [[0,0,0],[0,0,0]].
After applying first increment it becomes [[1,2,1],[0,1,0]].
The final matrix is [[1,3,1],[1,3,1]], which contains 6 odd numbers.

Example 2:

Input: m = 2, n = 2, indices = [[1,1],[0,0]]
Output: 0
Explanation: Final matrix = [[2,2],[2,2]]. There are no odd numbers in the final matrix.



Constraints:

1 <= m, n <= 50
1 <= indices.length <= 100
0 <= ri < m
0 <= ci < n



Follow up: Could you solve this in O(n + m + indices.length) time with only O(n + m) extra space?
*/

fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
	let mut res = vec![vec![0; n as usize]; m as usize];
	fn increment_row_and_column(m: usize, n: usize, res: &mut Vec<Vec<i32>>) {
		for i in 0..res[m].len() {
			res[m][i] += 1;
		}

		for i in 0..res.len() {
			res[i][n] += 1;
		}
	}
	for i in 0..indices.len() {
		increment_row_and_column(indices[i][0] as usize, indices[i][1] as usize, &mut res);
	}

	let mut count = 0;
	for i in 0..res.len() {
		for j in 0..res[i].len() {
			if res[i][j] % 2 == 1 {
				count += 1;
			}
		}
	}

	count
}


#[cfg(test)]
#[test]
fn test_odd_cells() {
	let m = 2;
	let n = 3;
	let indices = vec![vec![0, 1], vec![1, 1]];
	let result = 6;
	assert_eq!(odd_cells(m, n, indices), result);

	let m = 2;
	let n = 2;
	let indices = vec![vec![1, 1], vec![0, 0]];
	let result = 0;
	assert_eq!(odd_cells(m, n, indices), result);
}