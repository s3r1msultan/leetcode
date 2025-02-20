// A 3 x 3 magic square is a 3 x 3 grid filled with distinct numbers from 1 to 9 such that each row, column, and both diagonals all have the same sum.
//
// Given a row x col grid of integers, how many 3 x 3 contiguous magic square subgrids are there?
//
// Note: while a magic square can only contain numbers from 1 to 9, grid may contain numbers up to 15.
//
//
//
// Example 1:
//
// Input: grid = [[4,3,8,4],[9,5,1,9],[2,7,6,2]]
// Output: 1
// Explanation:
// The following subgrid is a 3 x 3 magic square:
//
// while this one is not:
//
// In total, there is only one magic square inside the given grid.
//
// Example 2:
//
// Input: grid = [[8]]
// Output: 0
//
//
//
// Constraints:
//
// row == grid.length
// col == grid[i].length
// 1 <= row, col <= 10
// 0 <= grid[i][j] <= 15
//
// fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
// 	let mut count = 0;
// 	let m = grid.len();
// 	let n = grid[0].len();
// 	let mut rows_sums = vec![];
// 	let mut cols_sums = vec![];
// 	let mut d1_sums = vec![];
// 	let mut d2_sums = vec![];
// 	for i in 0..m - 2 {
// 		for j in 0..n - 2 {
// 			for k in 0..3 {
// 				rows_sums.push(grid[i + k][j..j + 3]);
// 			}
// 		}
// 	}
//
// 	count
// }