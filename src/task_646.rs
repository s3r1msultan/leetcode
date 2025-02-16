/*You are given an array of n pairs pairs where pairs[i] = [lefti, righti] and lefti < righti.

A pair p2 = [c, d] follows a pair p1 = [a, b] if b < c. A chain of pairs can be formed in this fashion.

Return the length longest chain which can be formed.

You do not need to use up all the given intervals. You can select pairs in any order.



Example 1:

Input: pairs = [[1,2],[2,3],[3,4]]
Output: 2
Explanation: The longest chain is [1,2] -> [3,4].

Example 2:

Input: pairs = [[1,2],[7,8],[4,5]]
Output: 3
Explanation: The longest chain is [1,2] -> [4,5] -> [7,8].



Constraints:

n == pairs.length
1 <= n <= 1000
-1000 <= lefti < righti <= 1000

*/

fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
	let mut pairs = pairs;
	pairs.sort_unstable_by(|a, b| {
		if a[0] < b[1] {
			a[0].cmp(&b[1])
		} else {
			a[1].cmp(&b[0])
		}
	});
	let n = pairs.len();
	let mut dp = vec![1; n];

	for i in 1..n {
		let a = pairs[i][0];
		for j in 0..i {
			if a > pairs[j][1] {
				dp[i] = dp[i].max(dp[j] + 1);
			}
		}
	}

	println!("{dp:?}");

	*dp.iter().max().unwrap()
}