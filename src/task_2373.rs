// You are given an n x n integer matrix grid.
//
// Generate an integer matrix maxLocal of size (n - 2) x (n - 2) such that:
//
// maxLocal[i][j] is equal to the largest value of the 3 x 3 matrix in grid centered around row i + 1 and column j + 1.
//
// In other words, we want to find the largest value in every contiguous 3 x 3 matrix in grid.
//
// Return the generated matrix.
//
//
//
// Example 1:
//
// Input: grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
// Output: [[9,9],[8,6]]
// Explanation: The diagram above shows the original matrix and the generated matrix.
// Notice that each value in the generated matrix corresponds to the largest value of a contiguous 3 x 3 matrix in grid.
//
// Example 2:
//
// Input: grid = [[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]]
// Output: [[2,2,2],[2,2,2],[2,2,2]]
// Explanation: Notice that the 2 is contained within every contiguous 3 x 3 matrix in grid.
//
//
//
// Constraints:
//
// n == grid.length == grid[i].length
// 3 <= n <= 100
// 1 <= grid[i][j] <= 100
//


pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
	let n = grid.len();
	let mut result:Vec<Vec<i32>> = vec![vec![]; n-2];
	for i in 1..n-1 {
		for j in 1..n-1 {
			let mut max = 0;

			for m in i-1..=i+1 {
				for n in j-1..=j+1 {
					max = i32::max(max, grid[m][n]);
				}
			}

			result[i-1].push(max);
		}
	}
	result
}


#[cfg(test)]
#[test]

fn test_largest_local() {
	let grid = vec![vec![9,9,8,1],vec![5,6,2,6],vec![8,2,6,4],vec![6,2,2,2]];
	let result = vec![vec![9,9],vec![8,6]];
	assert_eq!(largest_local(grid), result);

	let grid = vec![vec![1,1,1,1,1],vec![1,1,1,1,1],vec![1,1,2,1,1],vec![1,1,1,1,1],vec![1,1,1,1,1]];
	let result = vec![vec![2,2,2],vec![2,2,2],vec![2,2,2]];
	assert_eq!(largest_local(grid), result);

}