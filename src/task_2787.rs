/* given two positive integers n and x.

return the number of ways n can be expressed as the sum of the xth power of unique positive integers, in other words, the number of sets of unique integers [n1, n2, ..., nk] where n = n1x + n2x + ... + nkx.

since the result can be very large, return it modulo 109 + 7.

for example, if n = 160 and x = 3, one way to express n is n = 23 + 33 + 53.



example 1:

input: n = 10, x = 2
output: 1
explanation: we can express n as the following: n = 32 + 12 = 10.
it can be shown that it is the only way to express 10 as the sum of the 2nd power of unique integers.

example 2:

input: n = 4, x = 1
output: 2
explanation: we can express n in the following ways:
- n = 41 = 4.
- n = 31 + 11 = 4.



constraints:

    1 <= n <= 300
    1 <= x <= 5

*/

pub fn number_of_ways(n: i32, x: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let x = x as u32;
    let n = n as usize;
    let mut dp = vec![vec![0; n + 1]; n + 1];

    dp[0][0] = 1;
    dp[1][1] = 1;

    for n in 1..=n {
        let val = (i as i64).pow(x);

        for j in 0..=n {
            dp[i][j] = dp[i - 1][j];
            if j >= val as usize {
                dp[i][j] = (dp[i][j] + dp[i - 1][j - val as usize]) % MOD;
            }
        }
    }

    dp[n][n]
}
