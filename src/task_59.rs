/*Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.



Example 1:

Input: n = 3
Output: [[1,2,3],[8,9,4],[7,6,5]]

Example 2:

Input: n = 1
Output: [[1]]



Constraints:

1 <= n <= 20

*/

fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
	let n = n as usize;
	let mut result = vec![vec![0; n]; n];
	let mut top = 0;
	let mut bottom = n;
	let mut left = 0;
	let mut right = n;
	let mut count = 1;

	while left < right && top < bottom {
		for i in left..right {
			result[top][i] = count;
			count += 1;
		}
		top += 1;

		for i in top..bottom {
			result[i][right - 1] = count;
			count += 1;
		}
		right -= 1;

		if top < bottom && left < right {
			for i in (left..right).rev() {
				result[bottom - 1][i] = count;
				count += 1;
			}
			bottom -= 1;

			for i in (top..bottom).rev() {
				result[i][left] = count;
				count += 1;
			}
			left += 1;
		}
	}

	result
}