/*Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.

A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.



Example 1:

Input: matrix = [[3,7,8],[9,11,13],[15,16,17]]
Output: [15]
Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column.

Example 2:

Input: matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
Output: [12]
Explanation: 12 is the only lucky number since it is the minimum in its row and the maximum in its column.

Example 3:

Input: matrix = [[7,8],[1,2]]
Output: [7]
Explanation: 7 is the only lucky number since it is the minimum in its row and the maximum in its column.



Constraints:

m == mat.length
n == mat[i].length
1 <= n, m <= 50
1 <= matrix[i][j] <= 105.
All elements in the matrix are distinct.

*/

fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
	let mut lucky_nums = vec![];
	let mut min_in_rows = vec![];
	let mut max_in_cols = vec![];
	for row in matrix.iter() {
		min_in_rows.push(*row.iter().min().unwrap());
	}

	for j in 0..matrix[0].len() {
		let mut max = 0;
		for i in 0..matrix.len() {
			max = max.max(matrix[i][j]);
		}
		max_in_cols.push(max);
	}

	for &row_num in min_in_rows.iter() {
		for &col_num in max_in_cols.iter() {
			if row_num == col_num {
				lucky_nums.push(row_num);
			}
		}
	}
	lucky_nums
}

#[cfg(test)]
#[test]
fn test_lucky_numbers() {
	let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
	let result = vec![15];
	assert_eq!(lucky_numbers(matrix), result);
}