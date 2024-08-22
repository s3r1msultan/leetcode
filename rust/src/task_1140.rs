/*Alice and Bob continue their games with piles of stones.  There are a number of piles arranged in a row, and each pile has a positive integer number of stones piles[i].  The objective of the game is to end with the most stones.

Alice and Bob take turns, with Alice starting first.  Initially, M = 1.

On each player's turn, that player can take all the stones in the first X remaining piles, where 1 <= X <= 2M.  Then, we set M = max(M, X).

The game continues until all the stones have been taken.

Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.



Example 1:

Input: piles = [2,7,9,4,4]
Output: 10
Explanation:  If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again. Alice can get 2 + 4 + 4 = 10 piles in total. If Alice takes two piles at the beginning, then Bob can take all three piles left. In this case, Alice get 2 + 7 = 9 piles in total. So we return 10 since it's larger.

Example 2:

Input: piles = [1,2,3,4,5,100]
Output: 104



Constraints:

1 <= piles.length <= 100
1 <= piles[i] <= 104

*/

fn stone_game_ii(piles: Vec<i32>) -> i32 {
	let n = piles.len();
	let mut m = 1;
	let mut dp = vec![vec![0; n]; n];
	let mut prefix_sum = vec![0; n + 1];
	for i in (0..n).rev() {
		prefix_sum[i] = prefix_sum[i + 1] + piles[i];
	}
	for i in 0..n {
		dp[i][n] = prefix_sum[i];
	}

	for index in (0..n).rev() {
		for max_till_now in (1..n).rev() {
			let mut x = 1;
			while x <= 2 * max_till_now && index + x <= n {
				dp[index][max_till_now] = dp[index][max_till_now].max(prefix_sum[index] - dp[index + x][max_till_now.max(x)]);
				x += 1;
			}
		}
	}

	dp[0][1]
}