/*You are given a 2D array of integers envelopes where envelopes[i] = [wi, hi] represents the width and the height of an envelope.

One envelope can fit into another if and only if both the width and height of one envelope are greater than the other envelope's width and height.

Return the maximum number of envelopes you can Russian doll (i.e., put one inside the other).

Note: You cannot rotate an envelope.



Example 1:

Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
Output: 3
Explanation: The maximum number of envelopes you can Russian doll is 3 ([2,3] => [5,4] => [6,7]).

Example 2:

Input: envelopes = [[1,1],[1,1],[1,1]]
Output: 1



Constraints:

1 <= envelopes.length <= 105
envelopes[i].length == 2
1 <= wi, hi <= 105

*/

fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
	let n = envelopes.len();
	let mut envelopes = envelopes;

	envelopes.sort_by(|a, b| {
		if a[1] == b[1] {
			b[0].cmp(&a[0])
		} else {
			a[1].cmp(&b[1])
		}
	});

	let widths: Vec<i32> = envelopes.iter().map(|e| e[0]).collect();
	let mut dp = vec![];
	for width in widths {
		match dp.binary_search(&width) {
			Ok(_) => {}
			Err(pos) => {
				if pos == dp.len() {
					dp.push(width);
				} else {
					dp[pos] = width;
				}
			}
		}
	}

	*dp.iter().max().unwrap()
}


#[cfg(test)]
#[test]
fn test_max_envelopes() {
	let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
	let result = 3;

	assert_eq!(max_envelopes(envelopes), 3);
}