/*Given an integer numRows, return the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:



Example 1:

Input: numRows = 5
Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]

Example 2:

Input: numRows = 1
Output: [[1]]



Constraints:

1 <= numRows <= 30

*/

fn generate(num_rows:i32) -> Vec<Vec<i32>> {
	if num_rows < 1 {
		return vec![];
	}

	let mut rows = vec![vec![1]];

	for i in 1..num_rows as usize {
		let mut row = vec![1];
		let prev_row = &rows[i-1];

		for j in 0..prev_row.len()-1 {
			row.push(prev_row[j]+prev_row[j+1]);
		}

		row.push(1);
		rows.push(row);
	}

	rows
}


#[cfg(test)]
#[test]
fn test_generate() {
	let n = 5;
	let result = vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]];
	assert_eq!(generate(n), result);

	let n = 1;
	let result = vec![vec![1]];
	assert_eq!(generate(n), result);

}