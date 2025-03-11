/*Given a binary string s, return the number of substrings with all characters 1's. Since the answer may be too large, return it modulo 109 + 7.



Example 1:

Input: s = "0110111"
Output: 9
Explanation: There are 9 substring in total with only 1's characters.
"1" -> 5 times.
"11" -> 3 times.
"111" -> 1 time.

Example 2:

Input: s = "101"
Output: 2
Explanation: Substring "1" is shown 2 times in s.

Example 3:

Input: s = "111111"
Output: 21
Explanation: Each substring contains only 1's characters.



Constraints:

1 <= s.length <= 105
s[i] is either '0' or '1'.

*/

pub fn num_sub(s: String) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = s.len();
    let chars = s.chars().collect::<Vec<_>>();
    let mut count = 0;
    let mut curr_count = 0;

    for right in 0..n {
        if chars[right] == '0' {
            curr_count = 0;
            continue;
        }

        curr_count = (curr_count + 1) % MOD;
        count = (count + curr_count) % MOD;
    }

    count
}