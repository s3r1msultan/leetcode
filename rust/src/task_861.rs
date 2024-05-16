// You are given an m x n binary matrix grid.
//
// A move consists of choosing any row or column and toggling each value in that row or column (i.e., changing all 0's to 1's, and all 1's to 0's).
//
// Every row of the matrix is interpreted as a binary number, and the score of the matrix is the sum of these numbers.
//
// Return the highest possible score after making any number of moves (including zero moves).
//
//
//
// Example 1:
//
// Input: grid = [[0,0,1,1],[1,0,1,0],[1,1,0,0]]
// Output: 39
// Explanation: 0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39
//
// Example 2:
//
// Input: grid = [[0]]
// Output: 1
//
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 20
// grid[i][j] is either 0 or 1.
//
pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
	let mut grid = grid;
	let mut result = 0;
	let m = grid.len();
	let n = grid[0].len();
	let mut ones = 0;
	for i in 0..m {
		if grid[i][0] == 0 {
			for j in 0..n {
				grid[i][j] = if grid[i][j]==0 {1} else {0};
			}
		}
	}

	ones=0;

	for i in 1..n {
		for j in 0..m {
			ones += grid[j][i];
		}

		if ones < m as i32 - ones {
			for j in 0..m {
				grid[j][i] = if grid[j][i]==0 {1} else {0};
			}
		}
		ones = 0;
	}

	for row in grid {
		result += row.into_iter()
			.rev()
			.enumerate()
			.map(|(bit, place)| place << bit)
			.sum::<i32>();
	}

	result
}

#[cfg(test)]
#[test]

fn test_matrix_score() {
	let grid = vec![vec![0,0,1,1],vec![1,0,1,0],vec![1,1,0,0]];
	let result = 39;
	assert_eq!(matrix_score(grid), result);

	let grid = vec![vec![0]];
	let result = 1;
	assert_eq!(matrix_score(grid), result);
}