/*Given an integer n, your task is to count how many strings of length n can be formed under the following rules:

Each character is a lower case vowel ('a', 'e', 'i', 'o', 'u')
Each vowel 'a' may only be followed by an 'e'.
Each vowel 'e' may only be followed by an 'a' or an 'i'.
Each vowel 'i' may not be followed by another 'i'.
Each vowel 'o' may only be followed by an 'i' or a 'u'.
Each vowel 'u' may only be followed by an 'a'.

Since the answer may be too large, return it modulo 10^9 + 7.



Example 1:

Input: n = 1
Output: 5
Explanation: All possible strings are: "a", "e", "i" , "o" and "u".

Example 2:

Input: n = 2
Output: 10
Explanation: All possible strings are: "ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" and "ua".

Example 3:

Input: n = 5
Output: 68



Constraints:

1 <= n <= 2 * 10^4

*/

pub fn count_vowel_permutation(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut dp = [1i64; 5];

    // 0 - 'a', 1 - 'e', 2 - 'i', 3 - 'o', 4 - 'u'

    for i in 0..n {
        let mut new_dp = [0i64; 5];
        new_dp[0] = (dp[1] + dp[2] + dp[4]) % MOD;
        new_dp[1] = (dp[0] + dp[2]) % MOD;
        new_dp[2] = (dp[1] + dp[3]) % MOD;
        new_dp[3] = dp[2];
        new_dp[4] = (dp[2] + dp[3]) % MOD;
        dp = new_dp;
    }

    let mut count = 0;
    for i in 0..5 {
        count += dp[i] % MOD;
        count %= MOD;
    }
    count as i32
}