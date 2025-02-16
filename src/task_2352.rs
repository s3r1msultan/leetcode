/*Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.

A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).



Example 1:

Input: grid = [[3,2,1],[1,7,6],[2,7,7]]
Output: 1
Explanation: There is 1 equal row and column pair:
- (Row 2, Column 1): [2,7,7]

Example 2:

Input: grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
Output: 3
Explanation: There are 3 equal row and column pairs:
- (Row 0, Column 0): [3,1,2,2]
- (Row 2, Column 2): [2,4,2,2]
- (Row 3, Column 2): [2,4,2,2]



Constraints:

n == grid.length == grid[i].length
1 <= n <= 200
1 <= grid[i][j] <= 105

*/
fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
	use std::collections::HashMap;

	let mut rows: HashMap<Vec<i32>, i32> = HashMap::new();
	let mut count = 0;

	for row in grid.iter() {
		*rows.entry(row.clone()).or_insert(0) += 1;
	}
	for j in 0..grid[0].len() {
		let mut column = vec![];
		for i in 0..grid.len() {
			column.push(grid[i][j]);
		}
		if rows.contains_key(&column) {
			count += rows.get(&column).unwrap();
		}
	}

	count
}