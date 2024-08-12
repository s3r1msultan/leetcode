/*Given an m x n matrix, return all elements of the matrix in spiral order.



Example 1:

Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
Output: [1,2,3,6,9,8,7,4,5]

Example 2:

Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
Output: [1,2,3,4,8,12,11,10,9,5,6,7]



Constraints:

m == matrix.length
n == matrix[i].length
1 <= m, n <= 10
-100 <= matrix[i][j] <= 100

*/

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
	let mut result = vec![];
	let mut top = 0;
	let mut left = 0;
	let mut bottom = matrix.len();
	let mut right = matrix[0].len();

	while top < bottom && left < right {
		for i in left..right {
			result.push(matrix[top][i]);
		}
		top += 1;

		for i in top..bottom {
			result.push(matrix[i][right - 1]);
		}
		right -= 1;

		if left < right && top < bottom {
			for i in (left..right).rev() {
				result.push(matrix[bottom][i]);
			}
			bottom -= 1;

			for i in (top..bottom).rev() {
				result.push(matrix[i][left]);
			}

			left += 1;
		}
	}

	result
}