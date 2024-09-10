/*You are given two integers m and n, which represent the dimensions of a matrix.

You are also given the head of a linked list of integers.

Generate an m x n matrix that contains the integers in the linked list presented in spiral order (clockwise), starting from the top-left of the matrix. If there are remaining empty spaces, fill them with -1.

Return the generated matrix.



Example 1:

Input: m = 3, n = 5, head = [3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]
Output: [[3, 0, 2, 6,8], [5, 0, - 1, - 1, 1], [5,2, 4, 9, 7]]
Explanation: The diagram above shows how the values are printed in the matrix.
Note that the remaining spaces in the matrix are filled with -1.

Example 2:

Input: m = 1, n = 4, head = [0, 1, 2]
Output: [[0, 1, 2, - 1]]
Explanation: The diagram above shows how the values are printed from left to right in the matrix.
The last space in the matrix is set to -1.


Constraints:

1 < = m, n < = 105
1 < = m * n < = 105
The number of nodes in the list is in the range [1, m * n].
0 < = Node.val < = 1000

*/
use crate::data_structures::list::ListNode;

pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
	let mut head = head;
	let m = m as usize;
	let n = n as usize;
	let mut top = 0;
	let mut bottom = m;
	let mut left = 0;
	let mut right = n;
	let mut matrix = vec![vec![-1; n]; m];
	while top < bottom && left < right {
		for i in left..right {
			if let Some(node) = head {
				matrix[top][i] = node.val;
				head = node.next;
			}
		}
		top += 1;

		for i in top..bottom {
			if let Some(node) = head {
				matrix[i][right - 1] = node.val;
				head = node.next;
			}
		}
		right -= 1;

		for i in (left..right).rev() {
			if let Some(node) = head {
				matrix[bottom - 1][i] = node.val;
				head = node.next;
			}
		}
		bottom -= 1;

		for i in (top..bottom).rev() {
			if let Some(node) = head {
				matrix[i][left] = node.val;
				head = node.next;
			}
		}
		left += 1;
	}
	matrix
}