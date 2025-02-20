/*Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:



Example 1:

Input: rowIndex = 3
Output: [1,3,3,1]

Example 2:

Input: rowIndex = 0
Output: [1]

Example 3:

Input: rowIndex = 1
Output: [1,1]



Constraints:

0 <= rowIndex <= 33



Follow up: Could you optimize your algorithm to use only O(rowIndex) extra space?
*/

fn get_row(row_index: i32) -> Vec<i32> {
	if row_index < 0 {
		return vec![];
	}

	let mut prev_row = vec![1];
	for i in 0..row_index {
		let mut curr_row = vec![1];
		for j in 0..prev_row.len()-1 {
			curr_row.push(prev_row[j] + prev_row[j+1]);
		}
		curr_row.push(1);
		prev_row = curr_row;
	}

	prev_row
}


#[cfg(test)]
#[test]
fn test_get_row() {
	let n = 3;
	let result = vec![1,3,3,1];
	assert_eq!(get_row(n), result);

	let n = 0;
	let result = vec![1];
	assert_eq!(get_row(n), result);

	let n = 1;
	let result = vec![1,1];
	assert_eq!(get_row(n), result);
}