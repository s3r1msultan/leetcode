/*You start at the cell (rStart, cStart) of an rows x cols grid facing east. The northwest corner is at the first row and column in the grid, and the southeast corner is at the last row and column.

You will walk in a clockwise spiral shape to visit every position in this grid. Whenever you move outside the grid's boundary, we continue our walk outside the grid (but may return to the grid boundary later.). Eventually, we reach all rows * cols spaces of the grid.

Return an array of coordinates representing the positions of the grid in the order you visited them.



Example 1:

Input: rows = 1, cols = 4, rStart = 0, cStart = 0
Output: [[0,0],[0,1],[0,2],[0,3]]

Example 2:

Input: rows = 5, cols = 6, rStart = 1, cStart = 4
Output: [[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]



Constraints:

1 <= rows, cols <= 100
0 <= rStart < rows
0 <= cStart < cols

*/

fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
	let mut result = vec![];
	let mut r = r_start;
	let mut c = c_start;
	let mut steps = 1;
	let mut direction = 0;
	let mut steps_count = 0;
	let mut steps_count_max = 0;
	let mut steps_count_max_change = false;

	while result.len() < (rows * cols) as usize {
		if r >= 0 && r < rows && c >= 0 && c < cols {
			result.push(vec![r, c]);
		}

		match direction {
			0 => c += 1,
			1 => r += 1,
			2 => c -= 1,
			3 => r -= 1,
			_ => (),
		}

		steps_count += 1;
		if steps_count == steps {
			steps_count = 0;
			direction = (direction + 1) % 4;
			if steps_count_max_change {
				steps_count_max += 1;
				steps_count_max_change = false;
			} else {
				steps_count_max_change = true;
			}
		}

		if steps_count_max == 2 {
			steps += 1;
			steps_count_max = 0;
		}
	}

	result
}

#[cfg(test)]
#[test]
fn test_spiral_matrix_iii() {
	let rows = 5;
	let cols = 6;
	let rStart = 1;
	let cStart = 4;
	let result = spiral_matrix_iii(rows, cols, rStart, cStart);
	println!("{:#?}", result);
}