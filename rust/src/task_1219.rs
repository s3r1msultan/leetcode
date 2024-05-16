// In a gold mine grid of size m x n, each cell in this mine has an integer representing the amount of gold in that cell, 0 if it is empty.
//
// Return the maximum amount of gold you can collect under the conditions:
//
// Every time you are located in a cell you will collect all the gold in that cell.
// From your position, you can walk one step to the left, right, up, or down.
// You can't visit the same cell more than once.
// Never visit a cell with 0 gold.
// You can start and stop collecting gold from any position in the grid that has some gold.
//
//
//
// Example 1:
//
// Input: grid = [[0,6,0],[5,8,7],[0,9,0]]
// Output: 24
// Explanation:
// [[0,6,0],
// [5,8,7],
// [0,9,0]]
// Path to get the maximum gold, 9 -> 8 -> 7.
//
// Example 2:
//
// Input: grid = [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
// Output: 28
// Explanation:
// [[1,0,7],
// [2,0,6],
// [3,4,5],
// [0,3,0],
// [9,0,20]]
// Path to get the maximum gold, 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7.
//
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 15
// 0 <= grid[i][j] <= 100
// There are at most 25 cells containing gold.
//

fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
	let m = grid.len();
	let n = grid[0].len();
	let mut max_gold = 0;

	fn dfs(grid: &Vec<Vec<i32>>, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, current_gold: i32) -> i32 {
		let m = grid.len();
		let n = grid[0].len();
		let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
		let mut local_max = current_gold;

		visited[x][y] = true;

		for &(dx, dy) in directions.iter() {
			let nx = x as i32 + dx;
			let ny = y as i32 + dy;

			if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] != 0 && !visited[nx as usize][ny as usize] {
				local_max = std::cmp::max(local_max, dfs(grid, nx as usize, ny as usize, visited, current_gold + grid[nx as usize][ny as usize]));
			}
		}

		visited[x][y] = false;
		local_max
	}

	for i in 0..m {
		for j in 0..n {
			if grid[i][j] != 0 {
				let mut visited = vec![vec![false; n]; m];
				max_gold = std::cmp::max(max_gold, dfs(&grid, i, j, &mut visited, grid[i][j]));
			}
		}
	}

	max_gold
}

#[cfg(test)]

#[test]

fn test_get_maximum_gold() {
	let grid = vec![vec![0,6,0],vec![5,8,7],vec![0,9,0]];
	let result = 24;
	assert_eq!(get_maximum_gold(grid), result);


	let grid = vec![vec![1,0,7],vec![2,0,6],vec![3,4,5],vec![0,3,0],vec![9,0,20]];
	let result = 28;
	assert_eq!(get_maximum_gold(grid), result);
}