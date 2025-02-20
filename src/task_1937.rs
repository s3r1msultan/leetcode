/*You are given an m x n integer matrix points (0-indexed). Starting with 0 points, you want to maximize the number of points you can get from the matrix.

To gain points, you must pick one cell in each row. Picking the cell at coordinates (r, c) will add points[r][c] to your score.

However, you will lose points if you pick a cell too far from the cell that you picked in the previous row. For every two adjacent rows r and r + 1 (where 0 <= r < m - 1), picking cells at coordinates (r, c1) and (r + 1, c2) will subtract abs(c1 - c2) from your score.

Return the maximum number of points you can achieve.

abs(x) is defined as:

x for x >= 0.
-x for x < 0.



Example 1:

Input: points = [[1,2,3],[1,5,1],[3,1,1]]
Output: 9
Explanation:
The blue cells denote the optimal cells to pick, which have coordinates (0, 2), (1, 1), and (2, 0).
You add 3 + 5 + 3 = 11 to your score.
However, you must subtract abs(2 - 1) + abs(1 - 0) = 2 from your score.
Your final score is 11 - 2 = 9.

Example 2:

Input: points = [[1,5],[2,3],[4,2]]
Output: 11
Explanation:
The blue cells denote the optimal cells to pick, which have coordinates (0, 1), (1, 1), and (2, 0).
You add 5 + 3 + 4 = 12 to your score.
However, you must subtract abs(1 - 1) + abs(1 - 0) = 1 from your score.
Your final score is 12 - 1 = 11.



Constraints:

m == points.length
n == points[r].length
1 <= m, n <= 105
1 <= m * n <= 105
0 <= points[r][c] <= 105

*/

fn max_points(points: Vec<Vec<i32>>) -> i64 {
	let mut previous_row: Vec<i64> = points[0].clone().iter().map(|&value| value as i64).collect();

	let rows = points.len();
	let cols = points[0].len();

	for i in 0..rows - 1 {
		let mut left_max: Vec<i64> = vec![0; cols];
		let mut right_max: Vec<i64> = vec![0; cols];
		let mut current_row: Vec<i64> = vec![0; cols];

		left_max[0] = previous_row[0];
		for j in 1..cols {
			left_max[j] = previous_row[j].max(left_max[j - 1] - 1);
		}

		right_max[cols - 1] = previous_row[cols - 1];
		for j in (0..cols - 1).rev() {
			right_max[j] = previous_row[j].max(right_max[j + 1] - 1);
		}

		for j in 0..cols {
			current_row[j] = points[i + 1][j] as i64 + right_max[j].max(left_max[j]);
		}

		previous_row = current_row;
	}
	let mut max_points: i64 = 0;
	for point in previous_row {
		max_points = max_points.max(point);
	}

	max_points
}