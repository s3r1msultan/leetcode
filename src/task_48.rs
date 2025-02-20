/*You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).

You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.



Example 1:

Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
Output: [[7,4,1],[8,5,2],[9,6,3]]

Example 2:

Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]



Constraints:

n == matrix.length == matrix[i].length
1 <= n <= 20
-1000 <= matrix[i][j] <= 1000

*/

fn rotate(matrix: &mut Vec<Vec<i32>>) {
	let n = matrix.len();
	for i in 0..n {
		for j in 0..n - i {
			let temp = matrix[i][j];
			matrix[i][j] = matrix[n - 1 - j][n - 1 - i];
			matrix[n - 1 - j][n - 1 - i] = temp;
		}
	}

	// Reverse each row
	for i in 0..n {
		matrix[i].reverse();
	}
}

#[cfg(test)]
#[test]
fn test_rotate() {
	let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
	let output = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
	rotate(&mut input);
	assert_eq!(input, output);
	let mut input = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];
	let output = vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]];
	rotate(&mut input);
	assert_eq!(input, output);
}